//! User use cases
#![allow(dead_code)]

use crate::ports::repositories::user::UserRepository;
use crate::ports::requests::user::{GetUserRequest, LoginRequest};
use crate::ports::responses::user::{GetUserResponse, GetUsersResponse, LoginResponse};
use crate::ports::services::user::UserService;
use std::error::Error;

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
    pub fn get_users(&self) -> Result<GetUsersResponse, Box<dyn Error>> {
        self.user_service.get_users()
    }

    /// Get a user
    // TODO: Add unit test
    pub fn get_user(&self, request: GetUserRequest) -> Result<GetUserResponse, Box<dyn Error>> {
        self.user_service.get_user(request)
    }

    /// Login
    // TODO: Add unit test
    pub fn login(&self, request: LoginRequest) -> Result<LoginResponse, Box<dyn Error>> {
        self.user_service.login(request)
    }
}
