//! Public Markdown twins for documentation and published articles.

use suprnova::FrameworkError;
use suprnova_markdown::MarkdownSource;

use crate::content::docs::chapter_markdown;
use crate::models::article::Article;

pub struct PublicMarkdown;

#[suprnova::__async_trait]
impl MarkdownSource for PublicMarkdown {
    async fn markdown(&self, path: &str) -> Result<Option<String>, FrameworkError> {
        if let Some(slug) = path.strip_prefix("/docs/") {
            return match chapter_markdown(slug) {
                Ok(markdown) => Ok(Some(markdown)),
                Err(error) if error.status_code() == 404 => Ok(None),
                Err(error) => Err(error),
            };
        }

        if let Some(slug) = path.strip_prefix("/blog/") {
            return Ok(Article::find_published_by_slug(slug)
                .await?
                .map(|article| article.body_markdown));
        }

        Ok(None)
    }
}
