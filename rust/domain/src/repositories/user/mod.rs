//! User repository

pub mod request;
pub mod response;

use crate::repositories::user::request::{
    AddUserScopeRepositoryRequest, GetUserScopesRepositoryRequest, RemoveUserScopeRepositoryRequest,
};
use crate::repositories::user::response::{
    AddUserScopeRepositoryResponse, GetUserScopesRepositoryResponse, RemoveUserScopeRepositoryResponse,
};
use crate::requests::user::{CreateUserRequest, DeleteUserRequest, UpdateUserPasswordRepositoryRequest};
use crate::{
    entities::user::User,
    requests::user::{LoginRequest, UserIdRequest},
};
use async_trait::async_trait;
use clean_architecture_shared::error::ApiResult;
use clean_architecture_shared::query_parameter::PaginateSort;

#[async_trait]
pub trait UserRepository {
    /// Get a list of users
    async fn get_users(&self, paginate_sort: &PaginateSort) -> ApiResult<Vec<User>>;

    /// Get a user by ID
    async fn get_user_by_id(&self, request: UserIdRequest) -> ApiResult<User>;

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

    /// Update user password
    async fn update_password(&self, request: UpdateUserPasswordRepositoryRequest) -> ApiResult<()>;

    /// Get user scopes
    async fn get_scopes(&self, request: GetUserScopesRepositoryRequest) -> ApiResult<GetUserScopesRepositoryResponse>;

    /// Add a scope to a user
    async fn add_scope(&self, request: AddUserScopeRepositoryRequest) -> ApiResult<AddUserScopeRepositoryResponse>;

    /// Remove a scope to a user
    async fn remove_scope(
        &self,
        request: RemoveUserScopeRepositoryRequest,
    ) -> ApiResult<RemoveUserScopeRepositoryResponse>;
}
