//! Category model.

use chrono::{DateTime, Utc};
use suprnova::eloquent::Model;
use suprnova::{FrameworkError, model};

#[model(
    table = "categories",
    fillable = [
        "name",
        "slug",
        "description",
        "sort_order",
        "is_visible"
    ],
    timestamps,
)]
pub struct Category {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub description: Option<String>,
    pub sort_order: i32,
    pub is_visible: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub use category::{ActiveModel, Column, Entity};

impl Category {
    pub async fn find_by_slug(slug: &str) -> Result<Option<Self>, FrameworkError> {
        <Self as Model>::query().filter("slug", slug).first().await
    }
}
