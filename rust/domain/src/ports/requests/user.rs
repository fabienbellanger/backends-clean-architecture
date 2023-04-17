//! User requests modules

use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct GetUserRequest {
    pub id: Uuid,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub lastname: String,
    pub firstname: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct DeleteUserRequest {
    pub id: Uuid,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct ForgottenPasswordRequest {
    #[validate(email)]
    pub email: String,
    pub expiration_duration: i64,
}
