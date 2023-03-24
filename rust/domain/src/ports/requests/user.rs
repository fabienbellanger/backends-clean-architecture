//! User requests modules

use validator::Validate;

#[derive(Debug, Validate)]
pub struct LoginRequest {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8))]
    pub password: String,
}

#[derive(Debug, Validate)]
pub struct GetUserRequest {
    #[validate(length(equal = 36))]
    pub id: String,
}
