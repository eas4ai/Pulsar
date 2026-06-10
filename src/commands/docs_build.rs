//! Documentation artifact build command.

use std::path::Path;

use async_trait::async_trait;
use suprnova::content::{DocsBuildConfig, DocsCatalog, build_docs};
use suprnova::{Command, FrameworkError, TypedCommand};
use tokio::fs;

#[derive(clap::Parser, Command, Debug)]
#[console(name = "docs:build", description = "Pre-render Pulsar documentation")]
pub struct DocsBuildCommand;

#[async_trait]
impl TypedCommand for DocsBuildCommand {
    async fn run(self) -> Result<(), FrameworkError> {
        let catalog =
            build_docs_from_paths(Path::new("content/docs"), Path::new("storage/content/docs"))
                .await?;
        println!("Built {} documentation chapters.", catalog.chapters.len());
        Ok(())
    }
}

/// Build the docs JSON artifacts from a Markdown source directory.
pub async fn build_docs_from_paths(
    source_dir: &Path,
    output_dir: &Path,
) -> Result<DocsCatalog, FrameworkError> {
    let toc_file = source_dir.join("documentation.md");

    validate_toc_entries(source_dir, &toc_file).await?;
    clear_output_files(output_dir).await?;

    build_docs(DocsBuildConfig {
        source_dir: source_dir.to_path_buf(),
        output_dir: output_dir.to_path_buf(),
        toc_file,
    })
    .await
    .map_err(|err| FrameworkError::internal(err.to_string()))
}

async fn validate_toc_entries(source_dir: &Path, toc_file: &Path) -> Result<(), FrameworkError> {
    let toc = fs::read_to_string(toc_file)
        .await
        .map_err(|err| path_error("read docs table of contents", toc_file, err))?;

    for target in toc.lines().filter_map(markdown_link_target) {
        if !target.ends_with(".md")
            || target.starts_with('/')
            || target.starts_with('#')
            || target.contains("://")
        {
            continue;
        }

        let chapter_path = source_dir.join(target);
        match fs::metadata(&chapter_path).await {
            Ok(meta) if meta.is_file() => {}
            Ok(_) => {
                return Err(FrameworkError::internal(format!(
                    "docs TOC entry `{target}` is not a file"
                )));
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                return Err(FrameworkError::internal(format!(
                    "docs TOC references missing chapter `{target}`"
                )));
            }
            Err(err) => return Err(path_error("read docs chapter metadata", &chapter_path, err)),
        }
    }

    Ok(())
}

async fn clear_output_files(output_dir: &Path) -> Result<(), FrameworkError> {
    fs::create_dir_all(output_dir)
        .await
        .map_err(|err| path_error("create docs output directory", output_dir, err))?;

    let mut entries = fs::read_dir(output_dir)
        .await
        .map_err(|err| path_error("read docs output directory", output_dir, err))?;

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|err| path_error("read docs output entry", output_dir, err))?
    {
        let path = entry.path();
        let file_type = entry
            .file_type()
            .await
            .map_err(|err| path_error("read docs output file type", &path, err))?;

        if file_type.is_file() || file_type.is_symlink() {
            fs::remove_file(&path)
                .await
                .map_err(|err| path_error("remove stale docs output file", &path, err))?;
        }
    }

    Ok(())
}

fn markdown_link_target(line: &str) -> Option<&str> {
    let title_end = line.find(']')?;
    let path_start = line[title_end..].find('(')? + title_end + 1;
    let path_end = line[path_start..].find(')')? + path_start;
    Some(line[path_start..path_end].trim())
}

fn path_error(action: &str, path: &Path, err: std::io::Error) -> FrameworkError {
    FrameworkError::internal(format!("{action} `{}`: {err}", path.display()))
}
