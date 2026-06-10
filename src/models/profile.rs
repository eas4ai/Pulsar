//! Profile model.

use chrono::{DateTime, Utc};
use suprnova::eloquent::Model;
use suprnova::{FrameworkError, attrs, model};

use crate::models::user::User;

#[model(
    table = "profiles",
    fillable = [
        "user_id",
        "handle",
        "display_name",
        "bio",
        "avatar_url",
        "website_url",
        "github_url",
        "location",
        "timezone"
    ],
    timestamps,
)]
pub struct Profile {
    pub id: i64,
    pub user_id: i64,
    pub handle: String,
    pub display_name: String,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub website_url: Option<String>,
    pub github_url: Option<String>,
    pub location: Option<String>,
    pub timezone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub use profile::{ActiveModel, Column, Entity};

impl Profile {
    pub async fn find_by_handle(handle: &str) -> Result<Option<Self>, FrameworkError> {
        <Self as Model>::query()
            .filter("handle", handle)
            .first()
            .await
    }

    pub async fn find_by_user_id(user_id: i64) -> Result<Option<Self>, FrameworkError> {
        <Self as Model>::query()
            .filter("user_id", user_id)
            .first()
            .await
    }

    pub async fn ensure_for_user(user: &User) -> Result<Self, FrameworkError> {
        if let Some(profile) = Self::find_by_user_id(user.id).await? {
            return Ok(profile);
        }

        <Self as Model>::create(attrs! {
            user_id: user.id,
            handle: default_handle(user),
            display_name: user.name.clone(),
        })
        .await
    }
}

fn default_handle(user: &User) -> String {
    format!("user-{}", user.id)
}
