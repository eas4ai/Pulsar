//! Temporary V2 user admin access surface.

use suprnova::{HttpResponse, Request, Response, handler};

#[handler]
pub async fn index(_req: Request) -> Response {
    HttpResponse::bytes_body(
        b"Admin users foundation".to_vec(),
        "text/plain; charset=utf-8",
    )
    .ok()
}
