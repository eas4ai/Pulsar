//! Content moderation review queue.

use serde::Serialize;
use suprnova::eloquent::Model;
use suprnova::{InertiaProps, Request, Response, handler, inertia_response};

use crate::controllers::inertia_config;
use crate::models::article::Article;

#[derive(Serialize)]
pub struct ModerationItemRow {
    pub id: i64,
    pub title: String,
    pub slug: String,
    pub status: String,
    pub author_id: i64,
    pub updated_at: String,
}

#[derive(InertiaProps)]
pub struct ModerationIndexProps {
    pub items: Vec<ModerationItemRow>,
}

#[handler]
pub async fn index(req: Request) -> Response {
    let items = Article::query()
        .filter("status", "draft")
        .order_by_asc("id")
        .get()
        .await?
        .iter()
        .map(|article| ModerationItemRow {
            id: article.id,
            title: article.title.clone(),
            slug: article.slug.clone(),
            status: article.status.clone(),
            author_id: article.author_id,
            updated_at: article.updated_at.to_rfc3339(),
        })
        .collect::<Vec<_>>();

    inertia_response!(
        &req,
        "moderation/Index",
        ModerationIndexProps { items },
        inertia_config()
    )
}
