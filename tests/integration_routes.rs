//! Release route inventory for the Pulsar starter kit.

mod common;

use chrono::Utc;
use serde_json::{Value, json};
use suprnova::HasRoles;
use suprnova::eloquent::Model;

use common::{Client, setup};
use pulsar::commands::users_promote::seed_default_roles;
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

#[tokio::test]
async fn public_and_authenticated_routes_return_expected_statuses() {
    let mut harness = setup().await;
    seed_default_roles().await.expect("seed roles");

    let member = verified_user("Member Route", "member-routes@pulsar.test").await;
    member.assign_role("member").await.expect("assign member");
    let author = verified_user("Author Route", "author-routes@pulsar.test").await;
    author.assign_role("author").await.expect("assign author");

    let addr = harness.spawn_app().await;
    let mut guest = Client::new(addr);

    for path in ["/", "/docs", "/docs/getting-started", "/blog"] {
        let resp = guest.get(path).await;
        assert_eq!(resp.status, 200, "GET {path} should render: {}", resp.body);
    }

    let feed = guest.get("/feed.xml").await;
    assert_eq!(feed.status, 200, "GET /feed.xml should render");
    assert!(
        feed.headers
            .get("content-type")
            .is_some_and(|value| value.contains("application/rss+xml")),
        "feed content type: {:?}",
        feed.headers
    );

    let manifest = guest.get("/site.webmanifest").await;
    assert_eq!(manifest.status, 200, "GET /site.webmanifest should render");
    let manifest: Value = serde_json::from_str(&manifest.body).expect("manifest parses as JSON");
    assert_eq!(manifest["name"], "Pulsar");
    assert_eq!(manifest["short_name"], "Pulsar");
    assert_eq!(manifest["theme_color"], "#101828");
    assert_eq!(manifest["background_color"], "#ffffff");
    assert_eq!(manifest["display"], "standalone");

    let dashboard = guest.get("/dashboard").await;
    assert_eq!(dashboard.status, 302, "guest /dashboard redirects");
    assert_eq!(dashboard.location(), "/login");
    let profile = guest.get("/profile").await;
    assert_eq!(profile.status, 302, "guest /profile redirects");
    assert_eq!(profile.location(), "/login");
    let admin = guest.get("/admin/articles").await;
    assert_eq!(admin.status, 302, "guest admin redirects");
    assert_eq!(admin.location(), "/login");

    let mut member_client = Client::new(addr);
    login(&mut member_client, "member-routes@pulsar.test").await;
    let dashboard = member_client.get("/dashboard").await;
    assert_eq!(dashboard.status, 200, "member dashboard renders");
    let profile = member_client.get("/profile").await;
    assert_eq!(profile.status, 200, "member profile renders");
    let admin = member_client.get("/admin/articles").await;
    assert_eq!(admin.status, 403, "member lacks article authoring access");

    let mut author_client = Client::new(addr);
    login(&mut author_client, "author-routes@pulsar.test").await;
    let admin = author_client.get("/admin/articles").await;
    assert_eq!(
        admin.status, 200,
        "author can open article admin: {}",
        admin.body
    );
}
