//! Badge model.

use chrono::{DateTime, Utc};
use suprnova::eloquent::Model;
use suprnova::{FrameworkError, model};

#[model(
    table = "badges",
    fillable = [
        "key",
        "name",
        "description",
        "icon",
        "role_scoped"
    ],
    timestamps,
)]
pub struct Badge {
    pub id: i64,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub icon: Option<String>,
    pub role_scoped: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub use badge::{ActiveModel, Column, Entity};

impl Badge {
    pub async fn find_by_key(key: &str) -> Result<Option<Self>, FrameworkError> {
        <Self as Model>::query().filter("key", key).first().await
    }
}
