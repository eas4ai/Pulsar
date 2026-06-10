//! Article model.

use chrono::{DateTime, Utc};
use suprnova::eloquent::{Direction, Model};
use suprnova::{FrameworkError, attrs, model};

use crate::content::articles::{decode_tags, encode_tags, normalize_tags, render_article_content};

const STATUS_DRAFT: &str = "draft";
const STATUS_PUBLISHED: &str = "published";
const SOURCE_FIRST_PARTY: &str = "first_party";

#[model(
    table = "articles",
    fillable = [
        "title",
        "slug",
        "body_markdown",
        "body_html",
        "excerpt",
        "description",
        "plain_text",
        "author_id",
        "status",
        "source",
        "category",
        "tags",
        "has_code",
        "has_math",
        "published_at"
    ],
    timestamps,
)]
pub struct Article {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub body_markdown: String,
    pub body_html: String,
    pub excerpt: String,
    pub description: String,
    pub plain_text: String,
    pub author_id: i64,
    pub status: String,
    pub source: String,
    pub category: String,
    pub tags: String,
    pub has_code: bool,
    pub has_math: bool,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub use article::{ActiveModel, Column, Entity};

#[derive(Clone, Debug)]
pub struct NewArticle {
    pub title: String,
    pub slug: String,
    pub body_markdown: String,
    pub author_id: i64,
    pub status: String,
    pub source: String,
    pub category: String,
    pub tags: Vec<String>,
    pub published_at: Option<DateTime<Utc>>,
}

impl Article {
    pub async fn create_from_markdown(input: NewArticle) -> Result<Self, FrameworkError> {
        let status = normalize_status(&input.status)?;
        let source = normalize_source(&input.source)?;
        let rendered = render_article_content(&input.title, &input.slug, &input.body_markdown)?;
        let published_at = resolve_published_at(&status, None, input.published_at);
        let tags = normalize_tags(&input.tags);

        <Self as Model>::create(attrs! {
            title: rendered.title,
            slug: rendered.slug,
            body_markdown: input.body_markdown,
            body_html: rendered.html,
            excerpt: rendered.excerpt,
            description: rendered.description,
            plain_text: rendered.plain_text,
            author_id: input.author_id,
            status: status,
            source: source,
            category: input.category.trim().to_string(),
            tags: encode_tags(&tags),
            has_code: rendered.has_code,
            has_math: rendered.has_math,
            published_at: published_at,
        })
        .await
    }

    pub async fn update_from_markdown(
        &mut self,
        input: ArticleUpdate,
    ) -> Result<(), FrameworkError> {
        let status = normalize_status(&input.status)?;
        let rendered = render_article_content(&input.title, &input.slug, &input.body_markdown)?;
        let tags = normalize_tags(&input.tags);
        let published_at = resolve_published_at(&status, self.published_at, None);

        self.title = rendered.title;
        self.slug = rendered.slug;
        self.body_markdown = input.body_markdown;
        self.body_html = rendered.html;
        self.excerpt = rendered.excerpt;
        self.description = rendered.description;
        self.plain_text = rendered.plain_text;
        self.status = status;
        self.category = input.category.trim().to_string();
        self.tags = encode_tags(&tags);
        self.has_code = rendered.has_code;
        self.has_math = rendered.has_math;
        self.published_at = published_at;

        <Self as Model>::save(self).await
    }

    pub async fn find_by_slug(slug: &str) -> Result<Option<Self>, FrameworkError> {
        <Self as Model>::query().filter("slug", slug).first().await
    }

    pub async fn find_published_by_slug(slug: &str) -> Result<Option<Self>, FrameworkError> {
        <Self as Model>::query()
            .filter("slug", slug)
            .filter("status", STATUS_PUBLISHED)
            .filter("source", SOURCE_FIRST_PARTY)
            .first()
            .await
    }

    pub async fn published() -> Result<Vec<Self>, FrameworkError> {
        Ok(<Self as Model>::query()
            .filter("status", STATUS_PUBLISHED)
            .filter("source", SOURCE_FIRST_PARTY)
            .order_by("published_at", Direction::Desc)
            .get()
            .await?
            .into_vec())
    }

    pub async fn all_for_admin() -> Result<Vec<Self>, FrameworkError> {
        Ok(<Self as Model>::query()
            .order_by("updated_at", Direction::Desc)
            .get()
            .await?
            .into_vec())
    }

    pub fn is_published(&self) -> bool {
        self.status == STATUS_PUBLISHED && self.published_at.is_some()
    }

    pub fn tags_vec(&self) -> Vec<String> {
        decode_tags(&self.tags)
    }

    pub fn set_tags(&mut self, tags: Vec<String>) {
        self.tags = encode_tags(&normalize_tags(&tags));
    }
}

#[derive(Clone, Debug)]
pub struct ArticleUpdate {
    pub title: String,
    pub slug: String,
    pub body_markdown: String,
    pub status: String,
    pub category: String,
    pub tags: Vec<String>,
}

fn normalize_status(status: &str) -> Result<String, FrameworkError> {
    let normalized = status.trim().to_ascii_lowercase();
    match normalized.as_str() {
        STATUS_DRAFT | STATUS_PUBLISHED => Ok(normalized),
        _ => Err(FrameworkError::bad_request(format!(
            "Unknown article status `{status}`. Expected draft or published."
        ))),
    }
}

fn normalize_source(source: &str) -> Result<String, FrameworkError> {
    let normalized = source.trim().to_ascii_lowercase();
    match normalized.as_str() {
        SOURCE_FIRST_PARTY | "user" | "aggregated" => Ok(normalized),
        _ => Err(FrameworkError::bad_request(format!(
            "Unknown article source `{source}`."
        ))),
    }
}

fn resolve_published_at(
    status: &str,
    existing: Option<DateTime<Utc>>,
    requested: Option<DateTime<Utc>>,
) -> Option<DateTime<Utc>> {
    if status == STATUS_PUBLISHED {
        requested.or(existing).or_else(|| Some(Utc::now()))
    } else {
        None
    }
}
