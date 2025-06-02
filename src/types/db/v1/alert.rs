use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

// System Alerts
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Alert {
    pub id: i64,
    pub alert_type: String,
    pub severity: String,
    pub service: String,
    pub message: String,
    pub timestamp: DateTime<Utc>,
    pub status: String,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<i64>,
    pub metadata: Option<serde_json::Value>,
    pub org_id: Option<i64>,
    pub app_id: Option<i64>,
    pub instance_id: Option<i64>,
    pub region_id: Option<i64>,
    pub node_id: Option<i64>,
}

// Alert Acknowledgments
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct AlertAcknowledgment {
    pub id: i64,
    pub alert_id: i64,
    pub user_id: i64,
    pub acknowledged_at: DateTime<Utc>,
    pub notes: Option<String>,
}

// Alert Escalations
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct AlertEscalation {
    pub id: i64,
    pub alert_id: i64,
    pub escalation_level: i64,
    pub escalated_at: DateTime<Utc>,
    pub escalated_to: serde_json::Value,
    pub escalation_method: String,
    pub response_required_by: Option<DateTime<Utc>>,
}

/// Represents an alert with all its related data (acknowledgments, escalations, and history).
/// This comprehensive view is useful for detailed alert pages.
#[derive(Debug, Serialize, Deserialize)]
pub struct AlertWithRelatedData {
    /// The core alert data
    pub alert: Alert,
    /// List of all acknowledgments for this alert
    pub acknowledgments: Vec<AlertAcknowledgment>,
    /// List of all escalations for this alert
    pub escalations: Vec<AlertEscalation>,
    /// History of all actions taken on this alert
    pub history: Vec<AlertHistory>
}

/// Represents an alert with its acknowledgment information.
/// This is useful for displaying alerts with their acknowledgment status.
#[derive(Debug, Serialize, Deserialize)]
pub struct AlertWithAcknowledgments {
    /// The core alert data
    pub alert: Alert,
    /// List of acknowledgments for this alert
    pub acknowledgments: Vec<AlertAcknowledgment>,
    /// Whether the alert has been acknowledged
    pub is_acknowledged: bool,
    /// Total number of acknowledgments
    pub acknowledgment_count: i64,
    /// Timestamp of the most recent acknowledgment, if any
    pub latest_acknowledgment: Option<chrono::DateTime<chrono::Utc>>,
}

// Alert History
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct AlertHistory {
    pub id: i64,
    pub alert_id: i64,
    pub action: String,
    pub performed_by: Option<i64>,
    pub performed_at: DateTime<Utc>,
    pub previous_state: Option<serde_json::Value>,
    pub new_state: Option<serde_json::Value>,
    pub notes: Option<String>,
}