use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Build {
    pub id: i64,
    pub app_id: i64,
    pub source_version: Option<String>,
    pub commit_sha: Option<String>,
    pub commit_message: Option<String>,
    pub author: Option<String>,
    pub status: String, // enum: 'pending', 'building', 'succeeded', 'failed', 'canceled'
    pub build_pack_used: Option<String>,
    pub build_pack_url: Option<String>,
    pub build_pack_version: Option<String>,
    pub build_image: Option<String>,
    pub build_arguments: Option<serde_json::Value>,
    pub build_environment: Option<serde_json::Value>,
    pub build_cache_key: Option<String>,
    pub log_url: Option<String>,
    pub artifact_url: Option<String>,
    pub artifact_checksum: Option<String>,
    pub artifact_size: Option<i64>,
    pub error_message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub build_duration: Option<i32>, // in seconds
    pub created_at: DateTime<Utc>,
}
