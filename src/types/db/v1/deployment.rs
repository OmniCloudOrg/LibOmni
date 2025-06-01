// models/deployment.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone, sqlx::FromRow)]
pub struct Deployment {
    pub id: i64,
    pub app_id: i64,
    pub build_id: i64,
    pub version: String,
    pub status: String,
    pub deployment_strategy: String,
    pub previous_deployment_id: Option<i64>,
    pub canary_percentage: Option<i64>,
    pub staged_instances: Option<i64>,
    pub total_instances: Option<i64>,
    pub environment_variables: Option<serde_json::Value>,
    pub annotations: Option<serde_json::Value>,
    pub labels: Option<serde_json::Value>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub deployment_duration: Option<i64>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<i64>,
}