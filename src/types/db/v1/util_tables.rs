use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use sqlx::FromRow;

/// Represents a resource type in the system.
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]

pub struct ResourceType {
    /// Unique identifier
    pub id: i32,
    /// Name of the resource type (e.g., 'cpu_usage', 'memory_usage')
    pub name: String,
    /// Category of the resource (e.g., 'compute', 'storage', 'network')
    pub category: String,
    /// Unit of measurement (e.g., 'vCPU-hour', 'GB-month')
    pub unit_of_measurement: String,
    /// Optional description of the resource type
    pub description: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}