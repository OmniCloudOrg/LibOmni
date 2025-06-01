use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::Row;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "ENUM", rename_all = "snake_case")]
pub enum WorkerStatus {
    Active,
    Provisioning,
    Maintenance,
    PoweredOff,
    Unreachable,
    Degraded,
    Decommissioning,
}

// Default implementation for WorkerStatus
impl Default for WorkerStatus {
    fn default() -> Self {
        WorkerStatus::Active
    }
}

// Function to provide default status for serde
fn default_status() -> WorkerStatus {
    WorkerStatus::Active
}

// Function to provide default SSH port
fn default_ssh_port() -> i32 {
    22
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Worker {
    pub id: Option<i64>,
    pub region_id: i64,
    pub name: String,
    pub provider_id: Option<String>,
    pub instance_type: Option<String>,
    pub status: String,
    pub cpu_total: f64,
    pub cpu_available: f64,
    #[serde(default)]
    pub cpu_reserved: f64,
    pub memory_total: f64,     // in MB
    pub memory_available: f64, // in MB
    #[serde(default)]
    pub memory_reserved: f64,  // in MB
    pub disk_total: f64,       // in MB
    pub disk_available: f64,   // in MB
    #[serde(default)]
    pub disk_reserved: f64,    // in MB
    pub network_in_capacity: Option<f64>,  // in Mbps
    pub network_out_capacity: Option<f64>, // in Mbps
    pub docker_version: Option<String>,
    pub ssh_address: Option<String>,
    #[serde(default = "default_ssh_port")]
    pub ssh_port: i32,
    pub ssh_user: Option<String>,
    pub ssh_key: Option<String>,
    pub labels: Option<serde_json::Value>,
    pub taints: Option<serde_json::Value>,
    pub annotations: Option<serde_json::Value>,
    pub last_heartbeat: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
