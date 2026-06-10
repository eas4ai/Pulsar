//! Runtime readers for generated documentation artifacts.

use std::path::{Path, PathBuf};

use serde::de::DeserializeOwned;
use suprnova::content::{DocsCatalog, DocsChapter};
use suprnova::{FrameworkError, serde_json};

const DOCS_OUTPUT_DIR: &str = "storage/content/docs";

pub fn load_catalog() -> Result<DocsCatalog, FrameworkError> {
    read_json(Path::new(DOCS_OUTPUT_DIR).join("catalog.json")).map_err(|err| match err {
        JsonReadError::NotFound(path) => FrameworkError::internal(format!(
            "docs catalog `{}` is missing; run `cargo run --bin console -- docs:build`",
            path.display()
        )),
        JsonReadError::Other(message) => FrameworkError::internal(message),
    })
}

pub fn load_chapter(slug: &str) -> Result<DocsChapter, FrameworkError> {
    if !valid_slug(slug) {
        return Err(FrameworkError::not_found(format!("docs chapter `{slug}`")));
    }

    read_json(Path::new(DOCS_OUTPUT_DIR).join(format!("{slug}.json"))).map_err(|err| match err {
        JsonReadError::NotFound(_) => FrameworkError::not_found(format!("docs chapter `{slug}`")),
        JsonReadError::Other(message) => FrameworkError::internal(message),
    })
}

pub fn first_chapter_slug(catalog: &DocsCatalog) -> Result<&str, FrameworkError> {
    catalog
        .chapters
        .first()
        .map(|entry| entry.slug.as_str())
        .ok_or_else(|| FrameworkError::not_found("docs chapter"))
}

fn valid_slug(slug: &str) -> bool {
    !slug.is_empty()
        && slug
            .chars()
            .all(|ch| ch.is_ascii_alphanumeric() || ch == '-')
}

fn read_json<T: DeserializeOwned>(path: PathBuf) -> Result<T, JsonReadError> {
    let raw = std::fs::read_to_string(&path).map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            JsonReadError::NotFound(path.clone())
        } else {
            JsonReadError::Other(format!("read docs artifact `{}`: {err}", path.display()))
        }
    })?;

    serde_json::from_str(&raw).map_err(|err| {
        JsonReadError::Other(format!("parse docs artifact `{}`: {err}", path.display()))
    })
}

enum JsonReadError {
    NotFound(PathBuf),
    Other(String),
}
