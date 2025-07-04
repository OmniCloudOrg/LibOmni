use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Org {
    pub id: i64,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
