//! Article, blog, admin authoring, and RSS flow tests.

mod common;

use chrono::{Duration, Utc};
use serde_json::json;
use suprnova::HasRoles;
use suprnova::eloquent::Model;

use common::{Client, setup};
use pulsar::commands::users_promote::seed_default_roles;
use pulsar::models::article::{Article, NewArticle};
use pulsar::models::user::User;

async fn verified_user(name: &str, email: &str) -> User {
    let mut user = User::create(name, email, "secretpass")
        .await
        .expect("create user");
    user.email_verified_at = Some(Utc::now());
    Model::save(&user).await.expect("verify user");
    user
}

async fn login(client: &mut Client, email: &str) {
    let page = client.get("/login").await;
    assert_eq!(page.status, 200, "GET /login should set CSRF cookie");

    let resp = client
        .post_json(
            "/login",
            json!({
                "email": email,
                "password": "secretpass",
                "remember": false,
            }),
        )
        .await;
    assert_eq!(resp.status, 302, "login should redirect: {}", resp.body);
    assert_eq!(resp.location(), "/dashboard");
}

async fn seed_article(slug: &str, status: &str) -> Article {
    let author = verified_user(
        &format!("Author {slug}"),
        &format!("{slug}@articles.pulsar.test"),
    )
    .await;

    let published_at = if status == "published" {
        Some(Utc::now() - Duration::minutes(slug.len() as i64))
    } else {
        None
    };

    Article::create_from_markdown(NewArticle {
        title: format!("Article {slug}"),
        slug: slug.to_string(),
        body_markdown: format!(
            "# Article {slug}\n\nThis article links to [Pulsar](/docs) and includes `code`."
        ),
        author_id: author.id,
        status: status.to_string(),
        source: "first_party".to_string(),
        category: "Engineering".to_string(),
        tags: vec!["release".to_string(), "pulsar".to_string()],
        published_at,
    })
    .await
    .expect("seed article")
}

#[tokio::test]
async fn public_blog_shows_published_articles_and_hides_drafts() {
    let mut harness = setup().await;
    seed_article("published-post", "published").await;
    seed_article("draft-post", "draft").await;
    let addr = harness.spawn_app().await;
    let mut client = Client::new(addr);

    let index = client.get("/blog").await;
    assert_eq!(index.status, 200, "GET /blog should render: {}", index.body);
    assert!(index.body.contains("published-post"));
    assert!(!index.body.contains("draft-post"));

    let detail = client.get("/blog/published-post").await;
    assert_eq!(detail.status, 200);
    assert!(detail.body.contains("Article published-post"));

    let draft = client.get("/blog/draft-post").await;
    assert_eq!(draft.status, 404);
}

#[tokio::test]
async fn rss_feed_uses_rendered_html_cdata_for_published_articles() {
    let mut harness = setup().await;
    let author = verified_user("Feed Writer", "feed-writer@pulsar.test").await;
    Article::create_from_markdown(NewArticle {
        title: "RSS Rendering".to_string(),
        slug: "rss-rendering".to_string(),
        body_markdown:
            "# RSS Rendering\n\nA [link](/docs).\n\n```rust\nfn main() {}\n```\n\nMath: $E=mc^2$."
                .to_string(),
        author_id: author.id,
        status: "published".to_string(),
        source: "first_party".to_string(),
        category: "Engineering".to_string(),
        tags: vec!["rss".to_string()],
        published_at: Some(Utc::now()),
    })
    .await
    .expect("seed rss article");
    let addr = harness.spawn_app().await;
    let mut client = Client::new(addr);

    let feed = client.get("/feed.xml").await;
    assert_eq!(feed.status, 200);
    assert!(
        feed.headers
            .get("content-type")
            .is_some_and(|value| value.contains("application/rss+xml")),
        "feed content type: {:?}",
        feed.headers
    );
    assert!(feed.body.contains("<rss version=\"2.0\">"));
    assert!(feed.body.contains("<![CDATA["));
    assert!(feed.body.contains("<a href=\"/docs\""));
    assert!(feed.body.contains("language-rust"));
    assert!(feed.body.contains("data-math-style") || feed.body.contains("language-math"));
}

#[tokio::test]
async fn non_author_cannot_open_article_admin() {
    let mut harness = setup().await;
    seed_default_roles().await.expect("seed roles");
    let user = verified_user("Member User", "member@pulsar.test").await;
    user.assign_role("member").await.expect("assign member");
    let addr = harness.spawn_app().await;
    let mut client = Client::new(addr);
    login(&mut client, "member@pulsar.test").await;

    let resp = client.get("/admin/articles").await;
    assert_eq!(resp.status, 403);
}

#[tokio::test]
async fn contributor_cannot_open_existing_article_edit_route() {
    let mut harness = setup().await;
    seed_default_roles().await.expect("seed roles");
    let article = seed_article("contributor-edit-denied", "draft").await;
    let user = verified_user("Contributor Edit", "contributor-edit@pulsar.test").await;
    user.assign_role("contributor")
        .await
        .expect("assign contributor");
    let addr = harness.spawn_app().await;
    let mut client = Client::new(addr);
    login(&mut client, "contributor-edit@pulsar.test").await;

    let resp = client
        .get(&format!("/admin/articles/{}/edit", article.id))
        .await;
    assert_eq!(resp.status, 403);
}

#[tokio::test]
async fn contributor_cannot_open_article_admin_index_or_create_form() {
    let mut harness = setup().await;
    seed_default_roles().await.expect("seed roles");
    let user = verified_user("Contributor Admin", "contributor-admin@pulsar.test").await;
    user.assign_role("contributor")
        .await
        .expect("assign contributor");
    let addr = harness.spawn_app().await;
    let mut client = Client::new(addr);
    login(&mut client, "contributor-admin@pulsar.test").await;

    let index = client.get("/admin/articles").await;
    assert_eq!(index.status, 403);

    let create = client.get("/admin/articles/create").await;
    assert_eq!(create.status, 403);
}

#[tokio::test]
async fn contributor_cannot_store_admin_article() {
    let mut harness = setup().await;
    seed_default_roles().await.expect("seed roles");
    let user = verified_user("Contributor Store", "contributor-store@pulsar.test").await;
    user.assign_role("contributor")
        .await
        .expect("assign contributor");
    let addr = harness.spawn_app().await;
    let mut client = Client::new(addr);
    login(&mut client, "contributor-store@pulsar.test").await;

    let resp = client
        .post_json(
            "/admin/articles",
            json!({
                "title": "Contributor Store Denied",
                "slug": "",
                "category": "Engineering",
                "tags": "rbac",
                "status": "draft",
                "body_markdown": "# Contributor Store Denied\n\nThis should not be first-party admin CRUD.",
            }),
        )
        .await;
    assert_eq!(resp.status, 403);
}

#[tokio::test]
async fn contributor_cannot_update_existing_article() {
    let mut harness = setup().await;
    seed_default_roles().await.expect("seed roles");
    let article = seed_article("contributor-update-denied", "draft").await;
    let user = verified_user("Contributor Update", "contributor-update@pulsar.test").await;
    user.assign_role("contributor")
        .await
        .expect("assign contributor");
    let addr = harness.spawn_app().await;
    let mut client = Client::new(addr);
    login(&mut client, "contributor-update@pulsar.test").await;

    let resp = client
        .put_json(
            &format!("/admin/articles/{}", article.id),
            json!({
                "title": "Contributor Update Denied",
                "slug": "contributor-update-denied",
                "category": "Engineering",
                "tags": "rbac",
                "status": "draft",
                "body_markdown": "# Contributor Update Denied\n\nThis should not be writable.",
            }),
        )
        .await;
    assert_eq!(resp.status, 403);
}

#[tokio::test]
async fn moderator_cannot_open_existing_article_edit_route() {
    let mut harness = setup().await;
    seed_default_roles().await.expect("seed roles");
    let article = seed_article("moderator-edit-denied", "draft").await;
    let user = verified_user("Moderator Edit", "moderator-edit@pulsar.test").await;
    user.assign_role("moderator")
        .await
        .expect("assign moderator");
    let addr = harness.spawn_app().await;
    let mut client = Client::new(addr);
    login(&mut client, "moderator-edit@pulsar.test").await;

    let resp = client
        .get(&format!("/admin/articles/{}/edit", article.id))
        .await;
    assert_eq!(resp.status, 403);
}

#[tokio::test]
async fn moderator_cannot_update_existing_article() {
    let mut harness = setup().await;
    seed_default_roles().await.expect("seed roles");
    let article = seed_article("moderator-update-denied", "draft").await;
    let user = verified_user("Moderator Update", "moderator-update@pulsar.test").await;
    user.assign_role("moderator")
        .await
        .expect("assign moderator");
    let addr = harness.spawn_app().await;
    let mut client = Client::new(addr);
    login(&mut client, "moderator-update@pulsar.test").await;

    let resp = client
        .put_json(
            &format!("/admin/articles/{}", article.id),
            json!({
                "title": "Moderator Update Denied",
                "slug": "moderator-update-denied",
                "category": "Engineering",
                "tags": "rbac",
                "status": "draft",
                "body_markdown": "# Moderator Update Denied\n\nThis should stay outside moderation review.",
            }),
        )
        .await;
    assert_eq!(resp.status, 403);
}

#[tokio::test]
async fn author_can_create_draft_publish_and_appear_on_blog() {
    let mut harness = setup().await;
    seed_default_roles().await.expect("seed roles");
    let user = verified_user("Author User", "author@pulsar.test").await;
    user.assign_role("author").await.expect("assign author");
    let addr = harness.spawn_app().await;
    let mut client = Client::new(addr);
    login(&mut client, "author@pulsar.test").await;

    let admin = client.get("/admin/articles").await;
    assert_eq!(
        admin.status, 200,
        "author admin should render: {}",
        admin.body
    );

    let create = client
        .post_json(
            "/admin/articles",
            json!({
                "title": "Launch Notes",
                "slug": "",
                "category": "Release",
                "tags": "pulsar, launch",
                "status": "draft",
                "body_markdown": "# Launch Notes\n\nPulsar now ships articles.",
            }),
        )
        .await;
    assert_eq!(
        create.status, 302,
        "create should redirect: {}",
        create.body
    );

    let draft_blog = client.get("/blog").await;
    assert!(!draft_blog.body.contains("launch-notes"));

    let article = Article::find_by_slug("launch-notes")
        .await
        .expect("lookup created article")
        .expect("created article exists");
    assert!(!article.has_code);
    assert!(!article.has_math);
    assert_eq!(article.source, "first_party");
    assert_eq!(article.tags_vec(), vec!["pulsar", "launch"]);

    let edit = client
        .get(&format!("/admin/articles/{}/edit", article.id))
        .await;
    assert_eq!(edit.status, 200, "author edit should render: {}", edit.body);

    let publish = client
        .post_json(
            &format!("/admin/articles/{}/publish", article.id),
            json!({}),
        )
        .await;
    assert_eq!(
        publish.status, 302,
        "publish should redirect: {}",
        publish.body
    );

    let blog = client.get("/blog").await;
    assert_eq!(blog.status, 200);
    assert!(blog.body.contains("launch-notes"));
    assert!(blog.body.contains("Launch Notes"));
}
