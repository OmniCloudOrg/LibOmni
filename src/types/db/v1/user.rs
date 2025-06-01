use sqlx::types::{chrono::NaiveDateTime, JsonValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use sqlx::types::Json;
use sqlx::Pool;
use sqlx::MySql;
use sqlx::FromRow;
use serde_json::Value; 
use sqlx::Row;
use jsonwebtoken::{decode, encode, DecodingKey, Validation, Algorithm};

use super::super::auth::{AuthConfig, Claims};

#[derive(Debug, FromRow, Serialize, Clone, Deserialize)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub email_verified: i8,
    pub password: String,
    pub salt: String,
    pub login_attempts: i64,
    pub active: bool,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_login_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserMeta {
    pub id: i64,
    pub user_id: i64,
    pub timezone: Option<String>,
    pub language: Option<String>,
    pub theme: Option<String>,
    pub notification_preferences: Option<serde_json::Value>,
    pub profile_image: Option<String>,
    pub dashboard_layout: Option<serde_json::Value>,
    pub onboarding_completed: i8,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserPii {
    pub id: i64,
    pub user_id: i64,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub full_name: Option<String>,
    pub identity_verified: i8,
    pub identity_verification_date: Option<DateTime<Utc>>,
    pub identity_verification_method: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserSession {
    pub id: i64,
    pub user_id: i64,
    pub session_token: String,
    pub refresh_token: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub device_info: Option<serde_json::Value>,
    pub location_info: Option<serde_json::Value>,
    pub is_active: i8,
    pub last_activity: Option<DateTime<Utc>>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

// Define a struct for session data
#[derive(Debug, sqlx::FromRow)]
struct SessionData {
    user_id: i64,
}

#[rocket::async_trait]
impl<'r> rocket::request::FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> rocket::request::Outcome<Self, Self::Error> {
        // Log the request path for context
        log::info!("Authentication attempt for path: {}", request.uri().path());
        
        // Get the authentication config
        let auth_config = match request.rocket().state::<AuthConfig>() {
            Some(config) => config,
            None => {
                log::error!("AuthConfig not found in rocket state");
                return rocket::request::Outcome::Forward(rocket::http::Status::InternalServerError)
            }
        };
        
        let pool = match request.rocket().state::<Pool<MySql>>() {
            Some(p) => p,
            None => {
                log::error!("Database pool not found in rocket state");
                return rocket::request::Outcome::Forward(rocket::http::Status::InternalServerError);
            }
        };
    
        // Check for Authorization header first (Bearer token)
        let token = if let Some(auth_header) = request.headers().get_one("Authorization") {
            if auth_header.starts_with("Bearer ") {
                log::info!("Found Bearer token in Authorization header");
                Some(auth_header.trim_start_matches("Bearer ").to_string())
            } else {
                log::info!("Authorization header present but not a Bearer token");
                None
            }
        } else {
            log::info!("No Authorization header found");
            None
        };
    
        // If no Authorization header, check for session_id cookie
        let user_id = if let Some(token_str) = token {
            // Validate the JWT token
            log::info!("Attempting JWT token validation");
            match validate_token(&token_str, auth_config) {
                Ok(claims) => {
                    // Extract user ID from sub claim
                    log::info!("JWT token validated successfully");
                    match claims.sub.parse::<i64>() {
                        Ok(id) => {
                            log::info!("User authenticated via JWT token, user_id: {}", id);
                            Some(id)
                        },
                        Err(e) => {
                            log::error!("Failed to parse user ID from token: {}", e);
                            None
                        }
                    }
                },
                Err(e) => {
                    log::error!("JWT validation failed: {}", e);
                    None
                }
            }
        } else if let Some(session_cookie) = request.cookies().get("session_id") {
            // Look up session in database using query_as with the SessionData struct
            log::info!("Attempting session cookie validation");
            match sqlx::query_as::<_, SessionData>(
                "SELECT user_id FROM user_sessions WHERE session_token = ? AND expires_at > NOW() AND is_active = 1"
            )
            .bind(session_cookie.value())
            .fetch_optional(pool)
            .await {
                Ok(Some(session)) => {
                    // Update last_activity
                    log::info!("Session found and valid for user_id: {}", session.user_id);
                    let _ = sqlx::query(
                        "UPDATE user_sessions SET last_activity = NOW() WHERE session_token = ?"
                    )
                    .bind(session_cookie.value())
                    .execute(pool)
                    .await;
                    
                    Some(session.user_id)
                },
                Ok(None) => {
                    log::warn!("Invalid or expired session: {}", session_cookie.value());
                    None
                },
                Err(e) => {
                    log::error!("Database error looking up session: {}", e);
                    None
                }
            }
        } else {
            // No authentication provided
            log::info!("No authentication method found");
            None
        };
    
        // Fetch the user if we have an ID
        if let Some(id) = user_id {
            // Use query_as to fetch the complete user
            log::info!("Fetching user details for user_id: {}", id);
            match sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE id = ?"
            )
            .bind(id)
            .fetch_one(pool)
            .await {
                Ok(user) => {
                    // Check if user is active
                    if user.active {
                        log::info!("User {} successfully authenticated and active", id);
                        rocket::request::Outcome::Success(user)
                    } else {
                        log::warn!("Inactive user attempted access: {}", id);
                        rocket::request::Outcome::Error((rocket::http::Status::Forbidden, ()))
                    }
                },
                Err(e) => {
                    log::error!("Error fetching user {}: {}", id, e);
                    rocket::request::Outcome::Error((rocket::http::Status::InternalServerError, ()))
                }
            }
        } else {
            // No valid authentication
            log::info!("Authentication failed, no valid credentials");
            rocket::request::Outcome::Forward(rocket::http::Status::Unauthorized)
        }
    }
}

// Token validation function
fn validate_token(token: &str, auth_config: &AuthConfig) -> Result<Claims, jsonwebtoken::errors::Error> {
    // Decode and validate the token
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(auth_config.jwt_secret.as_bytes()),
        &Validation::new(Algorithm::HS256)
    )?;
    
    // Return the claims
    Ok(token_data.claims)
}