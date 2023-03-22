//! User services module

use crate::{
    entities::error::UserError,
    ports::{
        repositories::user::UserRepository,
        requests::user::{GetUserRequest, LoginRequest},
        responses::user::{GetUserResponse, GetUsersResponse, LoginResponse},
    },
};

pub struct UserService<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> UserService<R> {
    /// Create a new service
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    /// Login
    // TODO: Add unit test
    pub async fn login(&self, request: LoginRequest) -> Result<LoginResponse, UserError> {
        self.user_repository
            .login(request)
            .await
            .map(|user| user.into())
    }

    /// Get all users
    // TODO: Add unit test
    pub async fn get_users(&self) -> Result<GetUsersResponse, UserError> {
        self.user_repository
            .get_users()
            .await
            .map(|users| users.into())
    }

    /// Get a user
    // TODO: Add unit test
    pub async fn get_user(&self, request: GetUserRequest) -> Result<GetUserResponse, UserError> {
        self.user_repository
            .get_user(request)
            .await
            .map(|user| user.into())
    }
}
