//! User use cases
#![allow(dead_code)]

use crate::ports::requests::user::CreateUserRequest;
use crate::ports::{
    repositories::user::UserRepository,
    requests::user::{GetUserRequest, LoginRequest},
    responses::user::{GetUserResponse, GetUsersResponse, LoginResponse},
    services::user::UserService,
};
use clean_architecture_shared::auth::Jwt;
use clean_architecture_shared::error::ApiResult;
use clean_architecture_shared::validation::validate_request_data;

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

    /// Get all users use case
    // TODO: Add unit test
    #[instrument(skip(self))]
    pub async fn get_users(&self) -> ApiResult<GetUsersResponse> {
        self.user_service.get_users().await
    }

    /// Get a user use case
    // TODO: Add unit test
    #[instrument(skip(self))]
    pub async fn get_user(&self, request: GetUserRequest) -> ApiResult<GetUserResponse> {
        validate_request_data(&request)?;

        self.user_service.get_user(request).await
    }

    /// Login use case
    // TODO: Add unit test
    #[instrument(skip(self))]
    pub async fn login(&self, request: LoginRequest, jwt: &Jwt) -> ApiResult<LoginResponse> {
        validate_request_data(&request)?;

        self.user_service.login(request, jwt).await
    }

    /// Create user use case
    // TODO: Add unit test
    #[instrument(skip(self))]
    pub async fn create_user(&self, request: CreateUserRequest) -> ApiResult<GetUserResponse> {
        validate_request_data(&request)?;

        self.user_service.create_user(request).await
    }
}
