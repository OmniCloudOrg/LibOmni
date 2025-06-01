use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::Row;

// Internal imports
use super::instance::Instance;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct App {
    pub id: i64,
    pub name: String,
    pub org_id: i64,
    pub git_repo: Option<String>,
    pub region_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub git_branch: Option<String>,
    pub maintenance_mode: bool,
    pub container_image_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AppWithInstanceCount {
    #[serde(flatten)]
    app_data: App,
    instance_count: i64,
}


// Define the struct with flattening
#[derive(Debug, Serialize)]
pub struct AppWithInstances {
    #[serde(flatten)]
    pub app: App,
    pub instances: Vec<Instance>,
}

impl<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> for AppWithInstanceCount {
    fn from_row(row: &'r sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(AppWithInstanceCount {
            app_data: App::from_row(row)?,
            instance_count: row.try_get::<i64, _>("instance_count")?,
        })
    }
}