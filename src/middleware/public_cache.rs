//! Prevent shared caches from storing response cookies.

use suprnova::{HttpResponse, Middleware, Next, Request, Response, async_trait};

const XSRF_COOKIE: &str = "XSRF-TOKEN";
const PRIVATE_CACHE_CONTROL: &str = "private, no-store";

/// Removes disposable framework cookies from public responses.
///
/// If a public response carries any other cookie, the cookie is preserved and
/// the response becomes private. This protects state-changing cookies such as
/// a rotated remember-me token.
pub struct PublicCacheCookies {
    droppable: [String; 2],
}

impl PublicCacheCookies {
    pub fn new(session_cookie: impl Into<String>) -> Self {
        Self {
            droppable: [session_cookie.into(), XSRF_COOKIE.to_string()],
        }
    }

    fn apply(&self, response: HttpResponse) -> HttpResponse {
        let is_public = response.header_values("Cache-Control").any(|line| {
            line.split(',')
                .any(|directive| directive.trim().eq_ignore_ascii_case("public"))
        });
        if !is_public || response.header_value("Set-Cookie").is_none() {
            return response;
        }

        let only_droppable = response
            .header_values("Set-Cookie")
            .all(|line| self.droppable.iter().any(|name| name == cookie_name(line)));

        if only_droppable {
            response.without_header("Set-Cookie")
        } else {
            response.replace_header("Cache-Control", PRIVATE_CACHE_CONTROL)
        }
    }
}

#[async_trait]
impl Middleware for PublicCacheCookies {
    async fn handle(&self, request: Request, next: Next) -> Response {
        match next(request).await {
            Ok(response) => Ok(self.apply(response)),
            Err(response) => Err(self.apply(response)),
        }
    }
}

fn cookie_name(line: &str) -> &str {
    line.split(';')
        .next()
        .and_then(|pair| pair.split_once('='))
        .map_or("", |(name, _)| name.trim())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn middleware() -> PublicCacheCookies {
        PublicCacheCookies::new("suprnova_session")
    }

    fn public_response() -> HttpResponse {
        HttpResponse::text("public").header("Cache-Control", "public, max-age=86400")
    }

    #[test]
    fn removes_session_and_csrf_cookies_from_public_responses() {
        let response = middleware().apply(
            public_response()
                .header("Set-Cookie", "XSRF-TOKEN=abc; Path=/")
                .header("Set-Cookie", "suprnova_session=xyz; Path=/; HttpOnly"),
        );

        assert_eq!(response.header_value("Set-Cookie"), None);
        assert_eq!(
            response.header_value("Cache-Control"),
            Some("public, max-age=86400")
        );
    }

    #[test]
    fn preserves_other_cookies_and_makes_the_response_private() {
        let response = middleware().apply(
            public_response()
                .header("Set-Cookie", "suprnova_session=xyz; Path=/")
                .header("Set-Cookie", "remember_me=token; Path=/"),
        );

        assert_eq!(response.header_values("Set-Cookie").count(), 2);
        assert_eq!(
            response.header_value("Cache-Control"),
            Some(PRIVATE_CACHE_CONTROL)
        );
    }

    #[test]
    fn leaves_non_public_responses_unchanged() {
        let response = HttpResponse::html("<html></html>")
            .header("Cache-Control", "no-cache")
            .header("Set-Cookie", "suprnova_session=xyz; Path=/");
        let response = middleware().apply(response);

        assert_eq!(
            response.header_value("Set-Cookie"),
            Some("suprnova_session=xyz; Path=/")
        );
        assert_eq!(response.header_value("Cache-Control"), Some("no-cache"));
    }
}
