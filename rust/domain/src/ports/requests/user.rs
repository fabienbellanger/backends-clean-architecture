//! User requests modules

#[derive(Debug)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct GetUserRequest {
    pub id: String,
}
