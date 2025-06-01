use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ProviderAuditLog {
    pub id: i64,
    pub provider_id: i64,
    pub action: String,
    pub details: Option<String>,
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