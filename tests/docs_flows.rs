//! Documentation route flow tests.

mod common;

use common::spawned_client;

#[tokio::test]
async fn docs_index_and_chapter_render_prebuilt_content() {
    let (_harness, mut client) = spawned_client().await;

    let index = client.get("/docs").await;
    assert_eq!(index.status, 200, "GET /docs should render: {}", index.body);
    assert!(index.body.contains("Getting Started"));
    assert!(index.body.contains("ports"));

    let chapter = client.get("/docs/getting-started").await;
    assert_eq!(
        chapter.status, 200,
        "GET /docs/getting-started should render: {}",
        chapter.body
    );
    assert!(chapter.body.contains("Getting Started"));
    assert!(chapter.body.contains("authentication"));

    let missing = client.get("/docs/does-not-exist").await;
    assert_eq!(missing.status, 404);
}
