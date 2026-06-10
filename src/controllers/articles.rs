//! Public blog/article controllers.

use chrono::{DateTime, Utc};
use serde::Serialize;
use suprnova::{FrameworkError, InertiaProps, Request, Response, handler, inertia_response};

use crate::controllers::inertia_config;
use crate::models::article::Article;

#[derive(Clone, Debug, Serialize)]
pub struct ArticleSummary {
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub category: String,
    pub tags: Vec<String>,
    pub published_at: String,
    pub has_code: bool,
    pub has_math: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct ArticleDetail {
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub body_html: String,
    pub category: String,
    pub tags: Vec<String>,
    pub published_at: String,
    pub has_code: bool,
    pub has_math: bool,
}

#[derive(InertiaProps)]
pub struct ArticlesIndexProps {
    pub articles: Vec<ArticleSummary>,
}

#[derive(InertiaProps)]
pub struct ArticleShowProps {
    pub article: ArticleDetail,
}

#[handler]
pub async fn index(req: Request) -> Response {
    let articles = Article::published()
        .await?
        .into_iter()
        .map(summary)
        .collect::<Vec<_>>();

    inertia_response!(
        &req,
        "articles/Index",
        ArticlesIndexProps { articles },
        inertia_config()
    )
}

#[handler]
pub async fn show(req: Request) -> Response {
    let slug = req
        .param("slug")
        .map_err(|_| FrameworkError::param("slug"))?;
    let article = Article::find_published_by_slug(slug)
        .await?
        .ok_or_else(|| FrameworkError::not_found(format!("article `{slug}`")))?;

    inertia_response!(
        &req,
        "articles/Show",
        ArticleShowProps {
            article: detail(article)
        },
        inertia_config()
    )
}

pub fn summary(article: Article) -> ArticleSummary {
    let tags = article.tags_vec();
    ArticleSummary {
        title: article.title,
        slug: article.slug,
        excerpt: article.excerpt,
        category: article.category,
        tags,
        published_at: format_date(article.published_at),
        has_code: article.has_code,
        has_math: article.has_math,
    }
}

pub fn detail(article: Article) -> ArticleDetail {
    let tags = article.tags_vec();
    ArticleDetail {
        title: article.title,
        slug: article.slug,
        excerpt: article.excerpt,
        body_html: article.body_html,
        category: article.category,
        tags,
        published_at: format_date(article.published_at),
        has_code: article.has_code,
        has_math: article.has_math,
    }
}

fn format_date(value: Option<DateTime<Utc>>) -> String {
    value.map(|dt| dt.to_rfc3339()).unwrap_or_default()
}
