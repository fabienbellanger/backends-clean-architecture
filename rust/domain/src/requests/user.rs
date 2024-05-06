//! User requests

use crate::entities::scope::ScopeId;
use crate::entities::user::UserId;
use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct UserIdRequest {
    pub id: UserId,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub lastname: String,
    pub firstname: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
    pub scopes: Option<Vec<String>>,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct DeleteUserRequest {
    pub id: UserId,
    pub authenticated_user_id: UserId,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct ForgottenPasswordRequest {
    #[validate(email)]
    pub email: String,
    pub expiration_duration: i64,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct UpdateUserPasswordRequest {
    pub token: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct UpdateUserPasswordRepositoryRequest {
    pub id: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize, Clone)]
pub struct UserScopeRequest {
    pub user_id: UserId,
    pub scope_id: ScopeId,
}
