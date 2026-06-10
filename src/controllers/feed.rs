//! RSS feed controller.

use chrono::{DateTime, Utc};
use suprnova::{HttpResponse, Request, Response, handler};

use crate::controllers::app_url;
use crate::models::article::Article;

#[handler]
pub async fn rss(_req: Request) -> Response {
    let articles = Article::published().await?;
    let xml = render_rss(&app_url(), &articles);
    HttpResponse::bytes_body(xml.into_bytes(), "application/rss+xml; charset=utf-8").ok()
}

pub fn render_rss(base_url: &str, articles: &[Article]) -> String {
    let base = base_url.trim_end_matches('/');
    let mut items = String::new();

    for article in articles {
        let link = format!("{base}/blog/{}", article.slug);
        let pub_date = article
            .published_at
            .map(format_rss_date)
            .unwrap_or_else(|| Utc::now().to_rfc2822());
        items.push_str(&format!(
            "<item><title>{}</title><link>{}</link><guid>{}</guid><pubDate>{}</pubDate><description><![CDATA[{}]]></description></item>",
            escape_xml(&article.title),
            escape_xml(&link),
            escape_xml(&link),
            escape_xml(&pub_date),
            cdata(&article.body_html),
        ));
    }

    format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?><rss version=\"2.0\"><channel><title>Pulsar Articles</title><link>{}</link><description>Latest Pulsar articles</description>{}</channel></rss>",
        escape_xml(base),
        items
    )
}

pub fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn cdata(input: &str) -> String {
    input.replace("]]>", "]]]]><![CDATA[>")
}

fn format_rss_date(value: DateTime<Utc>) -> String {
    value.to_rfc2822()
}
