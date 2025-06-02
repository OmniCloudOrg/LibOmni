use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Backup {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: String,
    pub backup_type: String, // enum: 'PLATFORM', 'APPLICATION', 'PARTIAL'
    pub status: String, // enum: 'CREATING', 'AVAILABLE', 'RESTORING', 'FAILED', 'DELETED'
    pub format_version: String,
    pub source_environment: String,
    pub encryption_method: Option<String>,
    pub encryption_key_id: Option<i64>,
    pub size_bytes: Option<i64>,
    pub has_system_core: bool,
    pub has_directors: bool,
    pub has_orchestrators: bool,
    pub has_network_config: bool,
    pub has_app_definitions: bool,
    pub has_volume_data: bool,
    pub included_apps: Option<String>,
    pub included_services: Option<String>,
    pub last_validated_at: Option<DateTime<Utc>>,
    pub last_restored_at: Option<DateTime<Utc>>,
    pub restore_target_environment: Option<String>,
    pub restore_status: Option<String>,
    pub storage_location: String,
    pub manifest_path: String,
    pub metadata: Option<Value>,
}
