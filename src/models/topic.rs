//! Topic model.

use chrono::{DateTime, Utc};
use suprnova::eloquent::Model;
use suprnova::{FrameworkError, model};

#[model(
    table = "topics",
    fillable = [
        "name",
        "slug",
        "description",
        "sort_order",
        "is_visible"
    ],
    timestamps,
)]
pub struct Topic {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub is_visible: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub use topic::{ActiveModel, Column, Entity};

impl Topic {
    pub async fn find_by_slug(slug: &str) -> Result<Option<Self>, FrameworkError> {
        <Self as Model>::query().filter("slug", slug).first().await
    }
}
