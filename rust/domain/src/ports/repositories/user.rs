//! User repository module

use crate::{
    entities::user::User,
    ports::requests::user::{GetUserRequest, LoginRequest},
};
use std::error::Error;

pub trait UserRepository {
    /// Get a list of users
    fn get_users(&self) -> Result<Vec<User>, Box<dyn Error>>;

    /// Get a user
    fn get_user(&self, request: GetUserRequest) -> Result<User, Box<dyn Error>>;

    /// Login
    fn login(&self, request: LoginRequest) -> Result<User, Box<dyn Error>>;
}
