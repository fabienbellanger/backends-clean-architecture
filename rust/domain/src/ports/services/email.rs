//! Email service

use crate::entities::email::{Email, SmtpConfig};
use crate::ports::requests::email::ForgottenPasswordRequest;
use clean_architecture_shared::error::ApiResult;

#[derive(Clone)]
pub struct EmailService<T: Email> {
    email: T,
    config: SmtpConfig,
}

impl<T: Email> EmailService<T> {
    /// Create a new service
    pub fn new(email: T, config: SmtpConfig) -> Self {
        Self { email, config }
    }

    /// Send forgotten password request
    #[instrument(skip(self))]
    pub fn send_forgotten_password(&self, request: ForgottenPasswordRequest) -> ApiResult<()> {
        self.email.send(&self.config, &request.clone().try_into()?)
    }
}
