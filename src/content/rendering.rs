//! Shared Markdown rendering service for V2 content surfaces.

use suprnova::FrameworkError;
use suprnova::content::{Heading, MarkdownRenderer};

const EXCERPT_MAX_CHARS: usize = 240;
const DESCRIPTION_MAX_CHARS: usize = 200;

#[derive(Clone, Debug)]
pub struct RenderedContent {
    pub html: String,
    pub excerpt: String,
    pub description: String,
    pub plain_text: String,
    pub headings: Vec<Heading>,
    pub has_code: bool,
    pub has_math: bool,
}

pub fn render_content(markdown: &str) -> Result<RenderedContent, FrameworkError> {
    let rendered = MarkdownRenderer::default()
        .render(markdown)
        .map_err(|err| FrameworkError::internal(err.to_string()))?;
    let heading_title = first_h1_title(&rendered.headings).map(str::to_owned);
    let preview_text = strip_leading_heading(&rendered.plain_text, heading_title.as_deref());
    let excerpt = truncate_text(&preview_text, EXCERPT_MAX_CHARS);
    let description = description_from_markdown(markdown).unwrap_or_else(|| {
        heading_title
            .as_deref()
            .map(|title| truncate_text(title, DESCRIPTION_MAX_CHARS))
            .unwrap_or_default()
    });
    let has_code = has_code(markdown, &rendered.html);
    let has_math = has_math(markdown, &rendered.html);

    Ok(RenderedContent {
        html: rendered.html,
        excerpt,
        description,
        plain_text: rendered.plain_text,
        headings: rendered.headings,
        has_code,
        has_math,
    })
}

pub fn first_heading_title(rendered: &RenderedContent) -> Option<&str> {
    first_h1_title(&rendered.headings)
}

fn first_h1_title(headings: &[Heading]) -> Option<&str> {
    headings
        .iter()
        .find(|heading| heading.level == 1)
        .map(|heading| heading.title.trim())
        .filter(|title| !title.is_empty())
}

fn strip_leading_heading(plain_text: &str, heading_title: Option<&str>) -> String {
    let plain_text = normalize_whitespace(plain_text);
    let Some(title) = heading_title
        .map(str::trim)
        .filter(|title| !title.is_empty())
    else {
        return plain_text;
    };

    if plain_text == title {
        return String::new();
    }

    let Some(rest) = plain_text.strip_prefix(title) else {
        return plain_text;
    };
    let Some(first) = rest.chars().next() else {
        return String::new();
    };
    if !(first.is_whitespace() || matches!(first, ':' | '-' | '–' | '—')) {
        return plain_text;
    }

    rest.trim_start_matches(|ch: char| ch.is_whitespace() || matches!(ch, ':' | '-' | '–' | '—'))
        .to_string()
}

fn description_from_markdown(markdown: &str) -> Option<String> {
    let mut lines = markdown.lines().peekable();
    if lines.peek().is_some_and(|line| line.trim() == "---") {
        lines.next();
        for line in lines.by_ref() {
            if line.trim() == "---" {
                break;
            }
        }
    }

    let mut block = Vec::new();
    while let Some(line) = lines.next() {
        let trimmed = line.trim();
        if let Some(marker) = fence_marker(trimmed) {
            if let Some(description) = description_from_block(&block) {
                return Some(description);
            }
            block.clear();
            for fenced_line in lines.by_ref() {
                if fenced_line.trim_start().starts_with(marker) {
                    break;
                }
            }
            continue;
        }

        if trimmed.is_empty() {
            if let Some(description) = description_from_block(&block) {
                return Some(description);
            }
            block.clear();
            continue;
        }

        if is_atx_heading(trimmed) {
            if let Some(description) = description_from_block(&block) {
                return Some(description);
            }
            block.clear();
            continue;
        }

        block.push(line);
    }

    description_from_block(&block)
}

fn description_from_block(block: &[&str]) -> Option<String> {
    let markdown = block.join("\n");
    let markdown = markdown.trim();
    if markdown.is_empty() || is_footnote_definition(markdown) {
        return None;
    }

    let rendered = MarkdownRenderer::default().render(markdown).ok()?;
    let description = truncate_text(&rendered.plain_text, DESCRIPTION_MAX_CHARS);
    if description.is_empty() {
        None
    } else {
        Some(description)
    }
}

fn is_atx_heading(line: &str) -> bool {
    let hashes = line.chars().take_while(|ch| *ch == '#').count();
    (1..=6).contains(&hashes)
        && line
            .chars()
            .nth(hashes)
            .is_some_and(|ch| ch.is_whitespace())
}

fn is_footnote_definition(markdown: &str) -> bool {
    let trimmed = markdown.trim_start();
    trimmed.starts_with("[^") && trimmed.contains("]:")
}

fn fence_marker(trimmed: &str) -> Option<&'static str> {
    if trimmed.starts_with("```") {
        Some("```")
    } else if trimmed.starts_with("~~~") {
        Some("~~~")
    } else {
        None
    }
}

fn has_code(markdown: &str, html: &str) -> bool {
    markdown
        .lines()
        .any(|line| fence_marker(line.trim_start()).is_some())
        || html.contains("<pre")
}

fn has_math(markdown: &str, html: &str) -> bool {
    let non_code_markdown = markdown_without_code(markdown);

    html.contains("data-math-style")
        || html.contains("language-math")
        || has_math_code_fence(markdown)
        || has_tex_math_delimiters(&non_code_markdown)
        || has_dollar_math(&non_code_markdown)
}

fn has_math_code_fence(markdown: &str) -> bool {
    markdown.lines().any(|line| {
        let trimmed = line.trim_start().to_ascii_lowercase();
        trimmed.starts_with("```math") || trimmed.starts_with("~~~math")
    })
}

fn has_tex_math_delimiters(markdown: &str) -> bool {
    (markdown.contains(r"\(") && markdown.contains(r"\)"))
        || (markdown.contains(r"\[") && markdown.contains(r"\]"))
}

fn markdown_without_code(markdown: &str) -> String {
    let mut out = String::new();
    let mut open_fence = None;

    for line in markdown.lines() {
        let trimmed = line.trim_start();
        if let Some(marker) = open_fence {
            if trimmed.starts_with(marker) {
                open_fence = None;
            }
            out.push('\n');
            continue;
        }

        if let Some(marker) = fence_marker(trimmed) {
            open_fence = Some(marker);
            out.push('\n');
            continue;
        }

        out.push_str(&line_without_code_spans(line));
        out.push('\n');
    }

    out
}

fn line_without_code_spans(line: &str) -> String {
    let chars = line.chars().collect::<Vec<_>>();
    let mut out = String::new();
    let mut index = 0;

    while index < chars.len() {
        if chars[index] != '`' {
            out.push(chars[index]);
            index += 1;
            continue;
        }

        let ticks = count_run(&chars, index, '`');
        index += ticks;
        while index < chars.len() {
            if chars[index] == '`' && count_run(&chars, index, '`') == ticks {
                index += ticks;
                break;
            }
            index += 1;
        }
        out.push(' ');
    }

    out
}

fn count_run(chars: &[char], start: usize, needle: char) -> usize {
    let mut count = 0;
    let mut index = start;
    while chars.get(index).is_some_and(|ch| *ch == needle) {
        count += 1;
        index += 1;
    }
    count
}

fn has_dollar_math(markdown: &str) -> bool {
    let chars = markdown.chars().collect::<Vec<_>>();
    let mut index = 0;

    while index < chars.len() {
        if chars[index] != '$' || is_escaped(&chars, index) {
            index += 1;
            continue;
        }

        if chars.get(index + 1).is_some_and(|ch| *ch == '$') {
            if has_closing_double_dollar(&chars, index + 2) {
                return true;
            }
            index += 2;
            continue;
        }

        if chars
            .get(index + 1)
            .is_none_or(|ch| ch.is_whitespace() || *ch == '$')
        {
            index += 1;
            continue;
        }

        let mut closing = index + 1;
        while closing < chars.len() {
            if chars[closing] == '$'
                && !is_escaped(&chars, closing)
                && closing > index + 1
                && !chars[closing - 1].is_whitespace()
            {
                return true;
            }
            closing += 1;
        }

        index += 1;
    }

    false
}

fn has_closing_double_dollar(chars: &[char], start: usize) -> bool {
    let mut index = start;
    while index + 1 < chars.len() {
        if chars[index] == '$'
            && chars[index + 1] == '$'
            && !is_escaped(chars, index)
            && chars[start..index].iter().any(|ch| !ch.is_whitespace())
        {
            return true;
        }
        index += 1;
    }
    false
}

fn is_escaped(chars: &[char], index: usize) -> bool {
    let mut slash_count = 0;
    let mut cursor = index;
    while cursor > 0 && chars[cursor - 1] == '\\' {
        slash_count += 1;
        cursor -= 1;
    }
    slash_count % 2 == 1
}

fn truncate_text(text: &str, max_chars: usize) -> String {
    let text = normalize_whitespace(text);
    if text.chars().count() <= max_chars {
        return text;
    }

    let mut truncated = text
        .chars()
        .take(max_chars)
        .collect::<String>()
        .trim_end()
        .to_string();
    truncated.push_str("...");
    truncated
}

fn normalize_whitespace(text: &str) -> String {
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace(" .", ".")
        .replace(" ,", ",")
        .replace(" :", ":")
        .replace(" ;", ";")
        .replace(" !", "!")
        .replace(" ?", "?")
}

#[cfg(test)]
mod tests {
    use super::{first_heading_title, render_content};

    #[test]
    fn renders_gfm_tables() {
        let rendered =
            render_content("| Name | Language |\n| --- | --- |\n| Ada | Rust |\n| Grace | Vue |")
                .expect("render markdown table");

        assert!(rendered.html.contains("<table>"));
        assert!(rendered.html.contains("<th>Name</th>"));
        assert!(rendered.html.contains("<td>Rust</td>"));
        assert!(rendered.plain_text.contains("Name Language Ada Rust"));
    }

    #[test]
    fn detects_math_markers() {
        let rendered = render_content("Inline math $E = mc^2$.\n\n```math\nx^2 + y^2 = z^2\n```")
            .expect("render markdown math");

        assert!(rendered.has_math);
        assert!(
            rendered.html.contains("data-math-style") || rendered.html.contains("language-math")
        );
    }

    #[test]
    fn ignores_math_markers_inside_code() {
        let inline_code =
            render_content("Use `$PATH` and `$HOME`.").expect("render inline code markdown");
        assert!(!inline_code.has_math);

        let fenced_code = render_content("```sh\necho \"$PATH $HOME\"\n```")
            .expect("render fenced code markdown");
        assert!(!fenced_code.has_math);

        let actual_math = render_content("Einstein wrote $E = mc^2$.").expect("render math");
        assert!(actual_math.has_math);
    }

    #[test]
    fn code_only_content_does_not_use_code_as_description() {
        let rendered =
            render_content("# Example\n\n```rust\ncargo test\n```").expect("render code only");

        assert_eq!(rendered.description, "Example");
    }

    #[test]
    fn footnote_only_content_does_not_use_footnotes_as_description() {
        let rendered =
            render_content("[^n]: Footnote-only metadata.").expect("render footnote only");

        assert_eq!(rendered.description, "");
    }

    #[test]
    fn excerpt_omits_leading_h1_before_dash_delimiters() {
        let en_dash =
            render_content("# Overview\n\n– concise summary").expect("render en dash summary");
        assert_eq!(en_dash.excerpt, "concise summary");

        let em_dash =
            render_content("# Overview\n\n— concise summary").expect("render em dash summary");
        assert_eq!(em_dash.excerpt, "concise summary");
    }

    #[test]
    fn renders_footnotes_with_sanitized_metadata() {
        let rendered = render_content(
            "Pulsar supports notes.[^n]\n\n[^n]: Footnote text <script>alert('x')</script>.",
        )
        .expect("render markdown footnote");

        assert!(rendered.html.contains("data-footnote-ref"));
        assert!(rendered.html.contains("data-footnote-backref"));
        assert!(rendered.html.contains("Footnote text"));
        assert!(!rendered.html.contains("<script"));
    }

    #[test]
    fn assigns_stable_heading_ids() {
        let rendered = render_content(
            "## Release Notes\n\n## Release Notes\n\n### Rust & Vue\n\n# Final Title",
        )
        .expect("render markdown headings");

        let ids = rendered
            .headings
            .iter()
            .map(|heading| heading.id.as_str())
            .collect::<Vec<_>>();
        assert_eq!(
            ids,
            vec![
                "release-notes",
                "release-notes-2",
                "rust-vue",
                "final-title"
            ]
        );
        assert!(rendered.html.contains("id=\"release-notes-2\""));
    }

    #[test]
    fn extracts_description_from_first_body_paragraph() {
        let rendered = render_content(
            "# Platform Overview\n\nPulsar V2 foundations give members profiles, taxonomy, and shared rendering.\n\nMore details follow.",
        )
        .expect("render markdown description");

        assert_eq!(first_heading_title(&rendered), Some("Platform Overview"));
        assert_eq!(
            rendered.description,
            "Pulsar V2 foundations give members profiles, taxonomy, and shared rendering."
        );
        assert!(!rendered.excerpt.starts_with("Platform Overview"));
    }
}
