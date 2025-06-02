pub use super::v1::user::User;
use rocket::serde::{Deserialize, Serialize};



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

