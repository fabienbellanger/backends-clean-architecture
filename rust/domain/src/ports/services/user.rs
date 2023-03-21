//! User services module

use std::error::Error;

use crate::ports::{
    repositories::user::UserRepository,
    requests::user::{GetUserRequest, LoginRequest},
    responses::user::{GetUserResponse, GetUsersResponse, LoginResponse},
};

pub struct UserService<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> UserService<R> {
    /// Login
    // TODO: Add unit test
    pub fn login(&self, request: LoginRequest) -> Result<LoginResponse, Box<dyn Error>> {
        self.user_repository.login(request).map(|user| user.into())
    }

    /// Get all users
    // TODO: Add unit test
    pub fn get_users(&self) -> Result<GetUsersResponse, Box<dyn Error>> {
        self.user_repository.get_users().map(|users| users.into())
    }

    /// Get a user
    // TODO: Add unit test
    pub fn get_user(&self, request: GetUserRequest) -> Result<GetUserResponse, Box<dyn Error>> {
        self.user_repository
            .get_user(request)
            .map(|user| user.into())
    }
}
