//! Email service

use crate::ports::requests::user::ForgottenPasswordRequest;
use clean_architecture_shared::error::ApiResult;

pub trait EmailService {
    /// Send email for forgotten password
    fn forgotten_password(&self, request: ForgottenPasswordRequest, token: &str) -> ApiResult<()>;
}
