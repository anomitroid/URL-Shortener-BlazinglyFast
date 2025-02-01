use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlEntry {
    pub full_url: String,
    pub short_id: String,
    pub clicks: u32,
}