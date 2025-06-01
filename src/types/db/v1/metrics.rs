use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;
use sqlx::Row;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Metric {
    pub id: i64,
    pub app_id: Option<i64>,
    pub metric_name: String,
    pub metric_value: f64,
    pub labels: Option<Value>,
    pub timestamp: Option<NaiveDateTime>,
}