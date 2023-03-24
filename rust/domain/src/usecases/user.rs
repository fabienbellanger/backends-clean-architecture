//! User use cases
#![allow(dead_code)]

use crate::ports::{
    repositories::user::UserRepository,
    requests::user::{GetUserRequest, LoginRequest},
    responses::user::{GetUserResponse, GetUsersResponse, LoginResponse},
    services::user::UserService,
};
use clean_architecture_shared::error::ApiResult;

/// Create new user use case
pub struct UserUseCase<R: UserRepository> {
    user_service: UserService<R>,
}

impl<R> UserUseCase<R>
where
    R: UserRepository,
{
    /// Get all users
    // TODO: Add unit test
    pub async fn get_users(&self) -> ApiResult<GetUsersResponse> {
        self.user_service.get_users().await
    }

    /// Get a user
    // TODO: Add unit test
    pub async fn get_user(&self, request: GetUserRequest) -> ApiResult<GetUserResponse> {
        // TODO: Validate request (in service?)
        self.user_service.get_user(request).await
    }

    /// Login
    // TODO: Add unit test
    pub async fn login(&self, request: LoginRequest) -> ApiResult<LoginResponse> {
        // TODO: Validate request (in service?)
        self.user_service.login(request).await
    }
}
