use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize, Clone, Default)]
pub struct Platform {
    pub id: Option<i64>,
    pub name: String,
    pub description: String,
    pub table_name: Option<String>,
    pub subdomain: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}