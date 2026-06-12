//! Documentation route flow tests.

mod common;

use common::spawned_client;

#[tokio::test]
async fn docs_index_and_chapter_render_prebuilt_content() {
    let (_harness, mut client) = spawned_client().await;

    let index = client.get("/docs").await;
    assert_eq!(index.status, 200, "GET /docs should render: {}", index.body);
    assert!(index.body.contains("Getting Started"));
    assert!(index.body.contains("Project Structure"));
    assert!(index.body.contains("not Rustdoc"));

    let chapter = client.get("/docs/getting-started").await;
    assert_eq!(
        chapter.status, 200,
        "GET /docs/getting-started should render: {}",
        chapter.body
    );
    assert!(chapter.body.contains("Getting Started"));
    assert!(chapter.body.contains("What You Get"));

    let frontend = client.get("/docs/frontend").await;
    assert_eq!(
        frontend.status, 200,
        "GET /docs/frontend should render: {}",
        frontend.body
    );
    assert!(frontend.body.contains("Frontend and Design System"));
    assert!(frontend.body.contains("Generated Props"));

    let missing = client.get("/docs/does-not-exist").await;
    assert_eq!(missing.status, 404);
}
