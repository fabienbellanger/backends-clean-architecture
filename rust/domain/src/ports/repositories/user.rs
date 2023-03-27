//! User repository module

use crate::ports::requests::user::CreateUserRequest;
use crate::{
    entities::user::User,
    ports::requests::user::{GetUserRequest, LoginRequest},
};
use async_trait::async_trait;
use clean_architecture_shared::error::ApiResult;

#[async_trait]
pub trait UserRepository {
    /// Get a list of users
    async fn get_users(&self) -> ApiResult<Vec<User>>;

    /// Get a user
    async fn get_user(&self, request: GetUserRequest) -> ApiResult<User>;

    /// Login
    async fn login(&self, request: LoginRequest) -> ApiResult<User>;

    /// Create user
    async fn create_user(&self, request: CreateUserRequest) -> ApiResult<User>;
}
