use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use serde_json::Value;
use sqlx::Row;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct UserNotification {
    pub id: i64,
    pub user_id: i64,
    pub org_id: Option<i64>,
    pub app_id: Option<i64>,
    pub notification_type: String,
    pub message: String,
    pub read_status: bool,
    pub importance: String,
    pub action_url: Option<String>,
    pub action_label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

// Role Notifications
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct RoleNotification {
    pub id: i64,
    pub role_id: i64,
    pub org_id: Option<i64>,
    pub app_id: Option<i64>,
    pub notification_type: String,
    pub message: String,
    pub importance: String,
    pub action_url: Option<String>,
    pub action_label: Option<String>,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

// Notification Acknowledgments
#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct NotificationAcknowledgment {
    pub id: i64,
    pub user_id: i64,
    pub notification_id: Option<i64>,
    pub role_notification_id: Option<i64>,
    pub acknowledged_at: DateTime<Utc>
}

/// Represents a comprehensive view of a user's notifications with unread counts.
/// This is useful for providing notification center overviews.
#[derive(Debug, Serialize, Deserialize)]
pub struct NotificationWithCount {
    /// Direct notifications for the user
    pub user_notifications: Vec<UserNotification>,
    /// Role-based notifications applicable to the user
    pub role_notifications: Vec<RoleNotification>,
    /// User's acknowledgments of role notifications
    pub acknowledgments: Vec<NotificationAcknowledgment>,
    /// Count of unread direct user notifications
    pub unread_user_count: i64,
    /// Count of unacknowledged role notifications
    pub unacknowledged_role_count: i64,
    /// Total count of unread notifications (user + role)
    pub total_unread_count: i64
}

/// Represents a user's notifications including those from their roles.
/// This combines personal notifications with role-based ones.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserNotificationWithRoleNotifications {
    /// Direct notifications for the user
    pub user_notifications: Vec<UserNotification>,
    /// Role-based notifications applicable to the user
    pub role_notifications: Vec<RoleNotification>,
    /// User's acknowledgments of role notifications
    pub acknowledgments: Vec<NotificationAcknowledgment>
}

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Notification {
    pub id: i64,
    pub user_id: Option<i64>,
    pub org_id: Option<i64>,
    pub app_id: Option<i64>,
    pub notification_type: String, // enum: 'info', 'warning', 'error', 'success'
    pub message: String,
    pub read_status: bool,
    pub created_at: DateTime<Utc>,
}