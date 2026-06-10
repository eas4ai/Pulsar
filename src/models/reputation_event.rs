//! Reputation event model.

use chrono::{DateTime, Utc};
use suprnova::model;

#[model(
    table = "reputation_events",
    fillable = [
        "user_id",
        "event_type",
        "target_type",
        "target_id",
        "weight",
        "summary"
    ],
    timestamps,
)]
pub struct ReputationEvent {
    pub id: i64,
    pub user_id: i64,
    pub event_type: String,
    pub target_type: String,
    pub target_id: i64,
    pub weight: i32,
    pub summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub use reputation_event::{ActiveModel, Column, Entity};
