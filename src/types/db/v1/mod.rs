use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub salt: String,
    pub email: String,
    pub active: bool,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct App {
    pub id: i64,
    pub name: String,
    pub org_id: i64,
    pub git_repo: Option<String>,
    pub region_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub git_branch: Option<String>,
    pub maintenance_mode: bool,
    pub container_image_url: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Org {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Region {
    pub id: i64,
    pub name: String,
    pub provider: String, // enum in DB: 'kubernetes' or 'custom'
    pub status: String,   // enum in DB: 'active', 'maintenance', 'offline'
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Permission {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub resource_type: Option<String>,
}

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

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Deployment {
    pub id: i64,
    pub status: String, // enum: 'pending', 'in_progress', 'deployed', 'failed'
    pub app_id: i64,
    pub build_id: i64,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Instance {
    pub id: i64,
    pub app_id: i64,
    pub instance_type: String,
    pub guid: String,
    pub status: String, // enum: 'running', 'starting', 'stopping', 'stopped', 'crashed', 'terminated', 'unknown'
    pub container_id: Option<String>,
    pub container_ip: Option<String>,
    pub allocation_id: Option<i64>,
    pub node_id: Option<i64>,
    pub instance_index: i32,
    pub last_health_check: Option<DateTime<Utc>>,
    pub health_status: String, // enum: 'healthy', 'unhealthy', 'unknown'
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<f64>,
    pub disk_usage: Option<f64>,
    pub uptime: Option<i32>,
    pub restart_count: Option<i32>,
    pub last_restart_reason: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub stop_time: Option<DateTime<Utc>>,
    pub exit_code: Option<i32>,
    pub exit_reason: Option<String>,
    pub scheduler_metadata: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: i64,
    pub org_id: Option<i64>,
    pub action: String,
    pub user_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub resource_id: Option<String>,
    pub resource_type: String,
}
