//! Public discovery, crawler metadata, structured data, and Markdown twins.

mod common;

use chrono::Utc;

use common::{Client, setup, spawned_client};
use pulsar::models::article::{Article, NewArticle};
use pulsar::models::user::User;

async fn article(slug: &str, status: &str) {
    let author = User::create(
        format!("SEO author {slug}"),
        format!("{slug}@seo.pulsar.test"),
        "secretpass",
    )
    .await
    .expect("create article author");
    Article::create_from_markdown(NewArticle {
        title: format!("SEO article {slug}"),
        slug: slug.to_owned(),
        body_markdown: format!("# SEO article {slug}\n\nCrawler-readable body."),
        author_id: author.id,
        status: status.to_owned(),
        source: "first_party".to_owned(),
        category: "Engineering".to_owned(),
        tags: vec!["seo".to_owned(), "release".to_owned()],
        published_at: (status == "published").then(Utc::now),
    })
    .await
    .expect("create article");
}

#[tokio::test]
async fn public_documents_render_metadata_and_structured_data_on_the_server() {
    let (_harness, mut client) = spawned_client().await;

    let home = client.get("/").await;
    assert_eq!(home.status, 200);
    let head = home.body.split("</head>").next().expect("home head");
    assert!(head.contains("<title>Pulsar - Suprnova product-site starter</title>"));
    assert!(head.contains("<meta name=\"description\""));
    assert!(head.contains("<link rel=\"canonical\" href=\"http://pulsar.test/\""));
    assert!(head.contains("<meta property=\"og:type\" content=\"website\""));
    assert!(head.contains("<meta name=\"twitter:card\" content=\"summary_large_image\""));
    assert!(head.contains("<link rel=\"alternate\" type=\"application/rss+xml\""));
    assert!(head.contains("application/ld+json"));
    assert!(head.contains("\"@type\":\"WebSite\""));
    let schema_script = head
        .lines()
        .find(|line| line.contains("application/ld+json"))
        .expect("JSON-LD script");
    assert!(schema_script.contains("nonce="), "{schema_script}");

    let chapter = client.get("/docs/getting-started").await;
    assert_eq!(chapter.status, 200);
    let head = chapter.body.split("</head>").next().expect("chapter head");
    assert!(head.contains("<title>Getting Started · Pulsar</title>"));
    assert!(head.contains("/docs/getting-started\""));
    assert!(head.contains("\"@type\":\"TechArticle\""));
    assert!(head.contains("rel=\"alternate\" type=\"text/markdown\""));
    assert!(head.contains("/docs/getting-started.md\""));
}

#[tokio::test]
async fn discovery_endpoints_map_the_public_site() {
    let (_harness, mut client) = spawned_client().await;

    let sitemap = client.get("/sitemap.xml").await;
    assert_eq!(sitemap.status, 200, "{}", sitemap.body);
    assert!(sitemap.body.starts_with("<?xml"));
    assert!(sitemap.body.contains("/docs/getting-started</loc>"));
    assert!(sitemap.body.contains("<lastmod>"));

    let robots = client.get("/robots.txt").await;
    assert_eq!(robots.status, 200);
    assert!(robots.body.contains("Disallow: /admin"));
    assert!(
        robots
            .body
            .contains("Sitemap: http://pulsar.test/sitemap.xml")
    );

    let llms = client.get("/llms.txt").await;
    assert_eq!(llms.status, 200);
    assert!(llms.body.starts_with("# Pulsar"));
    assert!(llms.body.contains("/docs/getting-started.md)"));
    assert!(llms.body.contains("/llms-full.txt"));

    let full = client.get("/llms-full.txt").await;
    assert_eq!(full.status, 200);
    assert_eq!(
        full.headers.get("x-robots-tag").map(String::as_str),
        Some("noindex")
    );
    assert!(full.body.contains("<!-- /docs/getting-started -->"));
    assert!(full.body.contains("<!-- /docs/deployment -->"));
}

#[tokio::test]
async fn markdown_twins_expose_sources_without_leaking_drafts() {
    let mut harness = setup().await;
    article("published-seo", "published").await;
    article("draft-seo", "draft").await;
    let addr = harness.spawn_app().await;
    let mut client = Client::new(addr);

    let docs = client.get("/docs/getting-started.md").await;
    assert_eq!(docs.status, 200, "{}", docs.body);
    assert!(docs.headers["content-type"].starts_with("text/markdown"));
    assert_eq!(
        docs.headers.get("x-robots-tag").map(String::as_str),
        Some("noindex")
    );
    assert_eq!(
        docs.headers.get("cache-control").map(String::as_str),
        Some("public, max-age=0")
    );
    assert!(docs.body.contains("# Getting Started"));
    assert!(!docs.body.contains("<div id=\"app\""));

    let published = client.get("/blog/published-seo.md").await;
    assert_eq!(published.status, 200, "{}", published.body);
    assert_eq!(
        published.body,
        "# SEO article published-seo\n\nCrawler-readable body."
    );

    let draft = client.get("/blog/draft-seo.md").await;
    assert_eq!(draft.status, 404);
}

#[tokio::test]
async fn published_articles_render_blogposting_metadata() {
    let mut harness = setup().await;
    article("structured-seo", "published").await;
    let addr = harness.spawn_app().await;
    let mut client = Client::new(addr);

    let page = client.get("/blog/structured-seo").await;
    assert_eq!(page.status, 200, "{}", page.body);
    let head = page.body.split("</head>").next().expect("article head");
    assert!(head.contains("<title>SEO article structured-seo · Pulsar</title>"));
    assert!(head.contains("<meta property=\"og:type\" content=\"article\""));
    assert!(head.contains("\"@type\":\"BlogPosting\""));
    assert!(head.contains("\"datePublished\":"));
    assert!(head.contains("\"keywords\":[\"seo\",\"release\"]"));
    assert!(head.contains("\"author\":"));
    assert!(head.contains("\"name\":\"SEO author structured-seo\""));
    assert!(head.contains("/blog/structured-seo.md\""));
}
