use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlEntry {
    pub original_url: String,
    pub short_id: String,
    pub clicks: u64,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl fmt::Display for UrlEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Shortened: {} -> {} (Clicks: {})",
            self.short_id, self.original_url, self.clicks
        )
    }
}