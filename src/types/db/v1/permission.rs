use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Permission {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub resource_type: Option<String>,
}