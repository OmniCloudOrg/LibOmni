pub use super::v1::user::User;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use std::sync::Arc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use rocket::State;
use chrono::{DateTime, Utc, Duration};
use serde_json::json;
use rocket::serde::json::Json;
use rocket::{post, get, routes};


// JWT claims struct
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,         // Subject (user ID)
    pub exp: usize,          // Expiration time
    pub iat: usize,          // Issued at
    pub user_data: User,     // User data embedded in token
}

// Login request
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// Auth config
#[derive(Debug)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_expiry_hours: i64,
}

