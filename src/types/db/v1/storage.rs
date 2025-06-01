use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;
use sqlx::Row;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct StorageClass {
    pub id: i64,
    pub name: String,
    pub provisioner: String,
    pub reclaim_policy: String,        // enum: 'Delete', 'Retain' TODO: @tristanpoland add recycle
    pub volume_binding_mode: String,   // enum: 'Immediate', 'WaitForFirstConsumer'
    pub allow_volume_expansion: bool,
    pub storage_type: String,          // 'local-disk', 'local-resilient', 'distributed', 'geo-replicated'
    pub default_filesystem: String,    // enum: 'ext4', 'xfs', 'btrfs', 'zfs'
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct StorageVolume {
    pub id: i64,
    pub app_id: i64,
    pub name: String,
    pub size_gb: i64,
    pub storage_class: String,
    pub access_mode: String,       // enum: 'ReadWriteOnce', 'ReadOnlyMany', 'ReadWriteMany'
    pub status: String,            // enum: 'Provisioned', 'Bound', 'Mounted', 'Released', 'Deleting', 'Deleted'
    pub node_id: i64,
    pub encryption_enabled: bool,
    pub persistence_level: String, // enum: 'Basic', 'Enhanced', 'High', 'Maximum'
    pub write_concern: String,     // enum: 'WriteAcknowledged', 'WriteDurable', 'WriteReplicated', 'WriteDistributed'
    pub reclaim_policy: String,    // enum: 'Delete', 'Retain'
    pub filesystem_type: Option<String>,
    pub storage_class_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub snapshot_id: Option<i64>,
    pub mount_path: Option<String>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct StorageSnapshot {
    pub id: i64,
    pub volume_id: i64,
    pub name: String,
    pub size_gb: i64,
    pub created_at: DateTime<Utc>,
    pub status: String,   // enum: 'Creating', 'Available', 'Deleting', 'Deleted'
    pub description: Option<String>,
    pub retention_date: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct StorageMigration {
    pub id: i64,
    pub source_volume_id: i64,
    pub destination_volume_id: i64,
    pub migration_type: String, // enum: 'StorageClass', 'Node', 'Zone', 'Environment'
    pub status: String, // enum: 'Pending', 'Copying', 'Syncing', 'ReadyForCutover', 'Completed', 'Failed'
    pub progress_percent: i32,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_online: bool,
    pub error_message: Option<String>,
    pub created_by: String,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct StorageQosPolicy {
    pub id: i64,
    pub name: String,
    pub max_iops: Option<i32>,
    pub max_throughput_mbps: Option<i32>,
    pub burst_iops: Option<i32>,
    pub burst_duration_seconds: Option<i32>,
    pub latency_target_ms: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}