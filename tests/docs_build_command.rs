use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use pulsar::commands::docs_build::build_docs_from_paths;

#[tokio::test]
async fn docs_build_clears_output_writes_catalog_and_rejects_missing_chapters() {
    let root = temp_dir("pulsar-docs-build");
    let source = root.join("content/docs");
    let output = root.join("storage/content/docs");

    tokio::fs::create_dir_all(&source).await.unwrap();
    tokio::fs::create_dir_all(&output).await.unwrap();
    tokio::fs::write(output.join("stale.json"), "{}")
        .await
        .unwrap();
    tokio::fs::write(source.join("documentation.md"), "- [Missing](missing.md)\n")
        .await
        .unwrap();

    let err = build_docs_from_paths(&source, &output).await.unwrap_err();
    assert!(
        err.message().contains("missing.md"),
        "missing chapter error should name the missing file: {}",
        err.message()
    );

    tokio::fs::write(
        source.join("documentation.md"),
        "- [Getting Started](getting-started.md)\n- [Authentication](authentication.md)\n",
    )
    .await
    .unwrap();
    tokio::fs::write(
        source.join("getting-started.md"),
        "# Getting Started\n\nUse ports `8765` and `5765`.\n",
    )
    .await
    .unwrap();
    tokio::fs::write(source.join("authentication.md"), "# Authentication\n")
        .await
        .unwrap();

    build_docs_from_paths(&source, &output).await.unwrap();

    assert!(
        !output.join("stale.json").exists(),
        "docs build should clear stale output files"
    );
    let catalog = tokio::fs::read_to_string(output.join("catalog.json"))
        .await
        .unwrap();
    assert!(catalog.contains("Getting Started"));
    let chapter = tokio::fs::read_to_string(output.join("getting-started.json"))
        .await
        .unwrap();
    assert!(chapter.contains("8765"));

    let _ = tokio::fs::remove_dir_all(root).await;
}

fn temp_dir(prefix: &str) -> std::path::PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("{prefix}-{}-{nanos}", std::process::id()));
    if Path::new(&path).exists() {
        std::fs::remove_dir_all(&path).unwrap();
    }
    path
}
