use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use super::region::Region;
use serde_json::Value;
use sqlx::Row;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Provider {
    pub id: i64,
    pub name: String,
    pub display_name: String,
    pub provider_type: String, // enum in DB: 'kubernetes' or 'custom'
    pub status: String, // enum in DB: 'active', 'inactive', 'maintenance'
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct ProviderAuditLog {
    pub id: i64,
    pub provider_id: i64,
    pub action: String,
    pub details: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// List provider-regions.
///
/// This function fetches all regions from the database, paired with their providers and their binding table data.
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ProviderRegion {
    #[sqlx(flatten)]
    region: Region,
    provider_name: String,
    binding_status: String,
}
