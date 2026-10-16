//! XML sitemap for public pages, documentation, and published articles.

use suprnova::{HttpResponse, Request, Response, handler};

use crate::content::docs::{chapter_source_path, load_catalog};
use crate::controllers::app_url;
use crate::controllers::feed::escape_xml;
use crate::models::article::Article;

#[handler]
pub async fn index(_req: Request) -> Response {
    let base = app_url();
    let mut urls: Vec<SitemapUrl> = ["/", "/docs", "/blog", "/topics", "/members"]
        .into_iter()
        .map(|path| SitemapUrl {
            loc: format!("{base}{path}"),
            lastmod: None,
        })
        .collect();

    for chapter in load_catalog()?.chapters {
        urls.push(SitemapUrl {
            loc: format!("{base}/docs/{}", chapter.slug),
            lastmod: chapter_lastmod(&chapter.slug),
        });
    }
    for article in Article::published().await? {
        urls.push(SitemapUrl {
            loc: format!("{base}/blog/{}", article.slug),
            lastmod: Some(article.updated_at.format("%Y-%m-%d").to_string()),
        });
    }

    HttpResponse::bytes_body(
        render_sitemap(&urls).into_bytes(),
        "application/xml; charset=utf-8",
    )
    .header("Cache-Control", "public, max-age=3600")
    .ok()
}

struct SitemapUrl {
    loc: String,
    lastmod: Option<String>,
}

fn chapter_lastmod(slug: &str) -> Option<String> {
    let modified = std::fs::metadata(chapter_source_path(slug))
        .ok()?
        .modified()
        .ok()?;
    let stamp: chrono::DateTime<chrono::Utc> = modified.into();
    Some(stamp.format("%Y-%m-%d").to_string())
}

fn render_sitemap(urls: &[SitemapUrl]) -> String {
    let mut entries = String::new();
    for url in urls {
        entries.push_str("<url><loc>");
        entries.push_str(&escape_xml(&url.loc));
        entries.push_str("</loc>");
        if let Some(lastmod) = &url.lastmod {
            entries.push_str(&format!("<lastmod>{}</lastmod>", escape_xml(lastmod)));
        }
        entries.push_str("</url>");
    }
    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><urlset xmlns=\"http://www.sitemaps.org/schemas/sitemap/0.9\">{entries}</urlset>"
    )
}
