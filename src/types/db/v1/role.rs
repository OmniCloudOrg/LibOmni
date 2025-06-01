use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::{Row, FromRow};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
}