use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UrlEntry {
    pub id: i32,
    pub original_url: String,
    pub short_id: String,
    pub clicks: i64,
    pub created_at: DateTime<Utc>,
    pub qr_code: String,
}