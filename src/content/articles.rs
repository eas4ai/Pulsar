//! Article Markdown rendering and metadata helpers.

use suprnova::content::{MarkdownRenderer, slugify_heading};
use suprnova::{FrameworkError, serde_json};

#[derive(Clone, Debug)]
pub struct RenderedArticleContent {
    pub title: String,
    pub slug: String,
    pub html: String,
    pub excerpt: String,
    pub description: String,
    pub plain_text: String,
    pub has_code: bool,
    pub has_math: bool,
}

pub fn render_article_content(
    title: &str,
    slug: &str,
    markdown: &str,
) -> Result<RenderedArticleContent, FrameworkError> {
    let rendered = MarkdownRenderer::default()
        .render(markdown)
        .map_err(|err| FrameworkError::internal(err.to_string()))?;
    let derived_title = rendered
        .headings
        .iter()
        .find(|heading| heading.level == 1)
        .map(|heading| heading.title.trim())
        .filter(|title| !title.is_empty())
        .unwrap_or_else(|| title.trim());
    let title = if derived_title.is_empty() {
        "Untitled Article".to_string()
    } else {
        derived_title.to_string()
    };

    let slug = if slug.trim().is_empty() {
        slugify_heading(&title)
    } else {
        slugify_heading(slug)
    };

    let has_code = markdown.contains("```") || rendered.html.contains("<pre");
    let has_math = markdown.contains('$')
        || rendered.html.contains("data-math-style")
        || rendered.html.contains("language-math");

    Ok(RenderedArticleContent {
        title,
        slug,
        html: rendered.html,
        excerpt: rendered.excerpt.clone(),
        description: rendered.excerpt,
        plain_text: rendered.plain_text,
        has_code,
        has_math,
    })
}

pub fn normalize_tags(tags: impl IntoIterator<Item = impl AsRef<str>>) -> Vec<String> {
    let mut out = Vec::new();
    for tag in tags {
        let normalized = tag.as_ref().trim().to_ascii_lowercase();
        if normalized.is_empty() || out.iter().any(|existing| existing == &normalized) {
            continue;
        }
        out.push(normalized);
    }
    out
}

pub fn encode_tags(tags: &[String]) -> String {
    serde_json::to_string(tags).unwrap_or_else(|_| "[]".to_string())
}

pub fn decode_tags(raw: &str) -> Vec<String> {
    serde_json::from_str::<Vec<String>>(raw)
        .map(normalize_tags)
        .unwrap_or_default()
}

pub fn parse_tag_input(raw: &str) -> Vec<String> {
    normalize_tags(raw.split(','))
}
