//! User use cases
#![allow(dead_code)]

use crate::entities::error::UserError;
use crate::ports::repositories::user::UserRepository;
use crate::ports::requests::user::{GetUserRequest, LoginRequest};
use crate::ports::responses::user::{GetUserResponse, GetUsersResponse, LoginResponse};
use crate::ports::services::user::UserService;

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
    pub async fn get_users(&self) -> Result<GetUsersResponse, UserError> {
        self.user_service.get_users().await
    }

    /// Get a user
    // TODO: Add unit test
    pub async fn get_user(&self, request: GetUserRequest) -> Result<GetUserResponse, UserError> {
        self.user_service.get_user(request).await
    }

    /// Login
    // TODO: Add unit test
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, UserError> {
        self.user_service.login(request).await
    }
}
