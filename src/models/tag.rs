//! Tag model.

use chrono::{DateTime, Utc};
use suprnova::eloquent::Model;
use suprnova::{FrameworkError, model};

#[model(
    table = "tags",
    fillable = [
        "name",
        "slug"
    ],
    timestamps,
)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub use tag::{ActiveModel, Column, Entity};

impl Tag {
    pub async fn find_by_slug(slug: &str) -> Result<Option<Self>, FrameworkError> {
        <Self as Model>::query().filter("slug", slug).first().await
    }
}
