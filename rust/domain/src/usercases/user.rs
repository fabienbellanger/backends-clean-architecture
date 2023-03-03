//! User use cases
#![allow(dead_code)]

use std::error::Error;
use crate::entities::user;

trait User {
    fn get_users(&self) -> Result<Vec<user::User>, Box<dyn Error>> {
        unimplemented!()
    }
}

/// Create new user use case
struct UserUseCase {
    user_repository: String, // TODO: Change
}

impl User for UserUseCase {
    /// Get all users
    fn get_users(&self) -> Result<Vec<user::User>, Box<dyn Error>> {
        unimplemented!()
    }
}
