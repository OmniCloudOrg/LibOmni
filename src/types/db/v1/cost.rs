use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use sqlx::FromRow;

/// Represents a cost metric entry in the system.
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct CostMetric {
    /// Unique identifier
    pub id: i64,
    /// Resource type ID
    pub resource_type_id: i32,
    /// Provider ID
    pub provider_id: Option<i64>,
    /// Region ID
    pub region_id: Option<i64>,
    /// Application ID
    pub app_id: Option<i64>,
    /// Worker ID
    pub worker_id: Option<i64>,
    /// Organization ID
    pub org_id: Option<i64>,
    /// Start time of the usage period
    pub start_time: DateTime<Utc>,
    /// End time of the usage period
    pub end_time: DateTime<Utc>,
    /// Amount of resource used
    pub usage_quantity: f64,
    /// Cost per unit
    pub unit_cost: f64,
    /// Currency code (e.g., 'USD')
    pub currency: String,
    /// Total cost for this usage
    pub total_cost: f64,
    /// Discount percentage applied
    pub discount_percentage: Option<f64>,
    /// Reason for the discount
    pub discount_reason: Option<String>,
    /// Billing period (e.g., '2025-05')
    pub billing_period: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Represents a cost metric with its associated resource type information.
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct CostMetricWithType {
    /// Unique identifier
    pub id: i64,
    /// Resource type ID
    pub resource_type_id: i32,
    /// Provider ID
    pub provider_id: Option<i64>,
    /// Region ID
    pub region_id: Option<i64>,
    /// Application ID
    pub app_id: Option<i64>,
    /// Worker ID
    pub worker_id: Option<i64>,
    /// Organization ID
    pub org_id: Option<i64>,
    /// Start time of the usage period
    pub start_time: DateTime<Utc>,
    /// End time of the usage period
    pub end_time: DateTime<Utc>,
    /// Amount of resource used
    pub usage_quantity: f64,
    /// Cost per unit
    pub unit_cost: f64,
    /// Currency code (e.g., 'USD')
    pub currency: String,
    /// Total cost for this usage
    pub total_cost: f64,
    /// Discount percentage applied
    pub discount_percentage: Option<f64>,
    /// Reason for the discount
    pub discount_reason: Option<String>,
    /// Billing period (e.g., '2025-05')
    pub billing_period: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// Resource type name
    pub resource_type_name: String,
    /// Resource type category
    pub resource_type_category: String,
    /// Unit of measurement
    pub unit_of_measurement: String,
}

/// Represents a cost budget entry in the system.
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct CostBudget {
    /// Unique identifier
    pub id: i64,
    /// Organization ID
    pub org_id: i64,
    /// Application ID (optional)
    pub app_id: Option<i64>,
    /// Budget name
    pub budget_name: String,
    /// Budget amount
    pub budget_amount: f64,
    /// Currency code (e.g., 'USD')
    pub currency: String,
    /// Budget period type
    pub budget_period: String,
    /// Start date of the budget period
    pub period_start: DateTime<Utc>,
    /// End date of the budget period
    pub period_end: DateTime<Utc>,
    /// Alert threshold percentage
    pub alert_threshold_percentage: f64,
    /// Contacts to alert when threshold is reached (JSON)
    pub alert_contacts: String,
    /// Whether the budget is active
    pub is_active: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
    /// User ID who created the budget
    pub created_by: i64,
}

/// Represents a cost projection entry in the system.
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct CostProjection {
    /// Unique identifier
    pub id: i64,
    /// Organization ID
    pub org_id: i64,
    /// Application ID (optional)
    pub app_id: Option<i64>,
    /// Projection period type (e.g., 'monthly', 'quarterly')
    pub projection_period: String,
    /// Start date of the projection period
    pub start_date: DateTime<Utc>,
    /// End date of the projection period
    pub end_date: DateTime<Utc>,
    /// Projected cost amount
    pub projected_cost: f64,
    /// Currency code (e.g., 'USD')
    pub currency: String,
    /// Projection model used (e.g., 'linear', 'average_30d')
    pub projection_model: String,
    /// Confidence level of the projection
    pub confidence_level: Option<f64>,
    /// Additional metadata about the projection (JSON)
    pub metadata: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Represents a resource pricing entry in the system.
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct ResourcePricing {
    /// Unique identifier
    pub id: i64,
    /// Resource type ID
    pub resource_type_id: i32,
    /// Provider ID
    pub provider_id: i64,
    /// Region ID (optional)
    pub region_id: Option<i64>,
    /// Tier name (e.g., 'standard', 'premium')
    pub tier_name: String,
    /// Price per unit
    pub unit_price: f64,
    /// Currency code (e.g., 'USD')
    pub currency: String,
    /// When this pricing becomes effective
    pub effective_from: DateTime<Utc>,
    /// When this pricing expires (optional)
    pub effective_to: Option<DateTime<Utc>>,
    /// Pricing model (e.g., 'on-demand', 'reserved')
    pub pricing_model: String,
    /// Commitment period (e.g., '1-year', '3-year')
    pub commitment_period: Option<String>,
    /// Volume discount tiers (JSON)
    pub volume_discount_tiers: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

/// Represents a cost allocation tag in the system.
#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct CostAllocationTag {
    /// Unique identifier
    pub id: i64,
    /// Tag key
    pub tag_key: String,
    /// Tag value
    pub tag_value: String,
    /// Resource ID
    pub resource_id: i64,
    /// Resource type
    pub resource_type: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}