//! User use cases

use crate::ports::requests::user::{CreateUserRequest, DeleteUserRequest};
use crate::ports::{
    repositories::user::UserRepository,
    requests::user::{GetUserRequest, LoginRequest},
    responses::user::{GetUserResponse, GetUsersResponse, LoginResponse},
    services::user::UserService,
};
use clean_architecture_shared::auth::Jwt;
use clean_architecture_shared::error::ApiResult;
use clean_architecture_shared::query_parameter::PaginateSort;
use clean_architecture_shared::validation::validate_request_data;

#[derive(Clone)]
/// Create new user use case
pub struct UserUseCase<R: UserRepository> {
    user_service: UserService<R>,
}

impl<R> UserUseCase<R>
where
    R: UserRepository,
{
    /// Create a new use case
    pub fn new(user_service: UserService<R>) -> Self {
        Self { user_service }
    }

    /// Login
    #[instrument(skip(self))]
    pub async fn login(&self, request: LoginRequest, jwt: &Jwt) -> ApiResult<LoginResponse> {
        validate_request_data(&request)?;

        self.user_service.login(request, jwt).await
    }

    /// Get all users
    #[instrument(skip(self))]
    pub async fn get_users(&self, paginate_sort: &PaginateSort) -> ApiResult<GetUsersResponse> {
        self.user_service.get_users(paginate_sort).await
    }

    /// Get a user
    #[instrument(skip(self))]
    pub async fn get_user(&self, request: GetUserRequest) -> ApiResult<GetUserResponse> {
        validate_request_data(&request)?;

        self.user_service.get_user(request).await
    }

    /// Create user
    #[instrument(skip(self))]
    pub async fn create_user(&self, request: CreateUserRequest) -> ApiResult<GetUserResponse> {
        validate_request_data(&request)?;

        self.user_service.create_user(request).await
    }

    /// Delete a user
    #[instrument(skip(self))]
    pub async fn delete_user(&self, request: DeleteUserRequest) -> ApiResult<u64> {
        validate_request_data(&request)?;

        self.user_service.delete_user(request).await
    }
}
