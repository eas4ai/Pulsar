//! Server-rendered metadata for public pages.

use crate::controllers::{app_url, site_name};

pub const DEFAULT_DESCRIPTION: &str = "A production-ready Suprnova starter kit with accounts, documentation, publishing, community pages, and an Inertia/Vue frontend.";

pub fn site_description() -> String {
    std::env::var("SEO_DESCRIPTION").unwrap_or_else(|_| DEFAULT_DESCRIPTION.to_string())
}

pub fn social_image_url() -> String {
    let path = std::env::var("SEO_SOCIAL_IMAGE").unwrap_or_else(|_| "/social-card.jpg".into());
    if path.starts_with("http://") || path.starts_with("https://") {
        path
    } else {
        format!("{}/{}", app_url(), path.trim_start_matches('/'))
    }
}

pub fn social_image_dimensions() -> (u32, u32) {
    let width = std::env::var("SEO_SOCIAL_IMAGE_WIDTH")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(1280);
    let height = std::env::var("SEO_SOCIAL_IMAGE_HEIGHT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(720);
    (width, height)
}

pub struct PageMeta {
    title: String,
    description: String,
    path: String,
    og_type: &'static str,
    markdown: bool,
}

impl PageMeta {
    pub fn page(
        title: impl Into<String>,
        description: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            title: title.into(),
            description: description.into(),
            path: path.into(),
            og_type: "website",
            markdown: false,
        }
    }

    pub fn article(
        title: impl Into<String>,
        description: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        Self {
            og_type: "article",
            ..Self::page(title, description, path)
        }
    }

    pub fn with_markdown(mut self) -> Self {
        self.markdown = true;
        self
    }

    pub fn render(&self) -> String {
        let base = app_url();
        let base = base.trim_end_matches('/');
        let name = site_name();
        let url = format!("{base}{}", self.path);
        let image = social_image_url();
        let (width, height) = social_image_dimensions();
        let language = std::env::var("SEO_LANGUAGE").unwrap_or_else(|_| "en".into());
        let locale = std::env::var("SEO_LOCALE").unwrap_or_else(|_| "en_US".into());
        let theme = std::env::var("SEO_THEME_COLOR").unwrap_or_else(|_| "#0b1020".into());
        let color_scheme =
            std::env::var("SEO_COLOR_SCHEME").unwrap_or_else(|_| "dark light".into());

        let title = escape_attr(&self.title);
        let description = escape_attr(&self.description);
        let url = escape_attr(&url);
        let image = escape_attr(&image);
        let name = escape_attr(&name);

        let mut out = format!(
            "<meta name=\"application-name\" content=\"{name}\">\n\
             <meta name=\"robots\" content=\"index, follow\">\n\
             <meta name=\"referrer\" content=\"strict-origin-when-cross-origin\">\n\
             <meta name=\"theme-color\" content=\"{}\">\n\
             <meta name=\"color-scheme\" content=\"{}\">\n\
             <link rel=\"canonical\" href=\"{url}\">\n\
             <link rel=\"alternate\" type=\"application/rss+xml\" title=\"{name} RSS\" href=\"{base}/feed.xml\">\n\
             <meta property=\"og:type\" content=\"{}\">\n\
             <meta property=\"og:site_name\" content=\"{name}\">\n\
             <meta property=\"og:locale\" content=\"{}\">\n\
             <meta property=\"og:title\" content=\"{title}\">\n\
             <meta property=\"og:url\" content=\"{url}\">\n\
             <meta property=\"og:image\" content=\"{image}\">\n\
             <meta property=\"og:image:width\" content=\"{}\">\n\
             <meta property=\"og:image:height\" content=\"{}\">\n\
             <meta property=\"og:image:alt\" content=\"{name}\">\n\
             <meta name=\"twitter:card\" content=\"summary_large_image\">\n\
             <meta name=\"twitter:title\" content=\"{title}\">\n\
             <meta name=\"twitter:image\" content=\"{image}\">\n\
             <meta name=\"twitter:image:alt\" content=\"{name}\">\n\
             <meta http-equiv=\"content-language\" content=\"{}\">\n",
            escape_attr(&theme),
            escape_attr(&color_scheme),
            self.og_type,
            escape_attr(&locale),
            width,
            height,
            escape_attr(&language),
        );
        if !self.description.is_empty() {
            out.push_str(&format!(
                "<meta name=\"description\" content=\"{description}\">\n\
                 <meta property=\"og:description\" content=\"{description}\">\n\
                 <meta name=\"twitter:description\" content=\"{description}\">\n"
            ));
        }
        if let Ok(site) = std::env::var("SEO_TWITTER_SITE")
            && !site.trim().is_empty()
        {
            out.push_str(&format!(
                "<meta name=\"twitter:site\" content=\"{}\">\n",
                escape_attr(site.trim())
            ));
        }
        if self.markdown {
            out.push_str(&suprnova_markdown::alternate_link(&url));
            out.push('\n');
        }
        out
    }
}

pub fn strip_leading_title(excerpt: &str, title: &str) -> String {
    excerpt
        .strip_prefix(title)
        .map(|rest| rest.trim_start().to_string())
        .unwrap_or_else(|| excerpt.to_string())
}

pub(crate) fn escape_attr(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn escapes_untrusted_metadata_attributes() {
        let html = PageMeta::page("Title", "say \"hello\" & <script>", "/safe").render();
        assert!(!html.contains("<script>"));
        assert!(html.contains("&quot;hello&quot;"));
        assert!(html.contains("&amp;"));
    }

    #[test]
    fn strips_a_repeated_chapter_title() {
        assert_eq!(
            strip_leading_title("Routing Requests reach handlers.", "Routing"),
            "Requests reach handlers."
        );
    }
}
