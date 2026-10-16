//! Public blog/article controllers.

use chrono::{DateTime, Utc};
use serde::Serialize;
use suprnova::eloquent::Model;
use suprnova::{FrameworkError, InertiaProps, Request, Response, handler, inertia_response};

use crate::content::meta::PageMeta;
use crate::content::schema;
use crate::controllers::{inertia_config_titled, site_name, with_page_head};
use crate::models::article::Article;
use crate::models::profile::Profile;
use crate::models::user::User;

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

    let response = inertia_response!(
        &req,
        "articles/Index",
        ArticlesIndexProps { articles },
        inertia_config_titled("Blog")
    );
    with_page_head(
        response,
        &PageMeta::page(
            format!("Blog · {}", site_name()),
            format!(
                "Articles, release notes, and product guidance from {}.",
                site_name()
            ),
            "/blog",
        ),
        &[],
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
    let title = article.title.clone();
    let excerpt = article.excerpt.clone();
    let article_slug = article.slug.clone();
    let tags = article.tags_vec();
    let published = article.published_at.map(|value| value.to_rfc3339());
    let modified = article.updated_at.to_rfc3339();
    let author = match <User as Model>::find(article.author_id).await? {
        Some(user) => {
            let handle = Profile::find_by_user_id(user.id)
                .await?
                .map(|profile| profile.handle);
            Some((
                user.name.unwrap_or_else(|| "Pulsar contributor".into()),
                handle,
            ))
        }
        None => None,
    };
    let blocks = schema::article_blocks(
        &article_slug,
        &title,
        &excerpt,
        &tags,
        published.as_deref(),
        &modified,
        author
            .as_ref()
            .map(|(name, handle)| (name.as_str(), handle.as_deref())),
    );

    let response = inertia_response!(
        &req,
        "articles/Show",
        ArticleShowProps {
            article: detail(article)
        },
        inertia_config_titled(&title)
    );
    with_page_head(
        response,
        &PageMeta::article(
            format!("{title} · {}", site_name()),
            excerpt,
            format!("/blog/{article_slug}"),
        )
        .with_markdown(),
        &blocks,
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
