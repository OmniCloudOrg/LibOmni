use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::Row;


#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Instance {
    pub id: i64,
    pub app_id: i64,
    pub instance_type: String,
    pub guid: String,
    pub status: String, // enum: 'running', 'starting', 'stopping', 'stopped', 'crashed', 'terminated', 'unknown'
    pub region_id: i64,
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