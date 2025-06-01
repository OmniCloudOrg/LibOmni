use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;
use sqlx::Row;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Region {
    pub id: i64,
    pub name: String,
    pub provider: i64, // enum in DB: 'kubernetes' or 'custom'
    pub created_at: DateTime<Utc>,
}

