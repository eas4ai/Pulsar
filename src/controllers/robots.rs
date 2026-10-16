//! Robots policy generated from the configured public application URL.

use suprnova::{HttpResponse, Request, Response, handler};

use crate::controllers::app_url;

#[handler]
pub async fn index(_req: Request) -> Response {
    let body = format!(
        "User-agent: *\nAllow: /\nDisallow: /admin\nDisallow: /moderation\nDisallow: /dashboard\nDisallow: /profile\nDisallow: /login\nDisallow: /register\nDisallow: /notifications\n\nSitemap: {}/sitemap.xml\n",
        app_url()
    );
    HttpResponse::bytes_body(body.into_bytes(), "text/plain; charset=utf-8")
        .header("Cache-Control", "public, max-age=3600")
        .ok()
}
