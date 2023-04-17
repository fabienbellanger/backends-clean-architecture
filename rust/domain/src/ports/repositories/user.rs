//! User repository module

use crate::ports::requests::user::{CreateUserRequest, DeleteUserRequest};
use crate::{
    entities::user::User,
    ports::requests::user::{GetUserRequest, LoginRequest},
};
use async_trait::async_trait;
use clean_architecture_shared::error::ApiResult;
use clean_architecture_shared::query_parameter::PaginateSort;

#[async_trait]
pub trait UserRepository {
    /// Get a list of users
    async fn get_users(&self, paginate_sort: &PaginateSort) -> ApiResult<Vec<User>>;

    /// Get a user by ID
    async fn get_user_by_id(&self, request: GetUserRequest) -> ApiResult<User>;

    /// Get a user by email
    async fn get_user_by_email(&self, email: String) -> ApiResult<User>;

    /// Login
    async fn login(&self, request: LoginRequest) -> ApiResult<Option<User>>;

    /// Create user
    async fn create_user(&self, request: CreateUserRequest) -> ApiResult<User>;

    /// Delete user
    async fn delete_user(&self, request: DeleteUserRequest) -> ApiResult<u64>;

    /// Returns total number of users
    async fn get_total_users(&self) -> ApiResult<i64>;
}
