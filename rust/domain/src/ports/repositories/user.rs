//! User repository module

use crate::{
    entities::{error::UserError, user::User},
    ports::requests::user::{GetUserRequest, LoginRequest},
};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    /// Get a list of users
    async fn get_users(&self) -> Result<Vec<User>, UserError>;

    /// Get a user
    async fn get_user(&self, request: GetUserRequest) -> Result<User, UserError>;

    /// Login
    async fn login(&self, request: LoginRequest) -> Result<User, UserError>;
}
