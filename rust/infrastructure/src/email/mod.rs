//! Email module

mod forgotten_password;

use crate::config::Config;
use crate::email::forgotten_password::ForgottenPassword;
use clean_architecture_domain::ports::requests::user::ForgottenPasswordRequest;
use clean_architecture_domain::ports::services::email::EmailService;
use clean_architecture_shared::error::ApiResult;

#[derive(Debug, Default, Clone)]
pub struct EmailConfig {
    host: String,
    port: u16,
    timeout: u64,
    username: Option<String>,
    password: Option<String>,

    /// Forgotten password config
    forgotten_password_expiration_duration: i64,
    forgotten_password_base_url: String,
    forgotten_password_email_from: String,
}

impl From<Config> for EmailConfig {
    fn from(config: Config) -> Self {
        EmailConfig {
            host: config.smtp_host,
            port: config.smtp_port,
            timeout: config.smtp_timeout,
            username: None, // TODO
            password: None, // TODO
            forgotten_password_expiration_duration: config.forgotten_password_expiration_duration,
            forgotten_password_base_url: config.forgotten_password_base_url,
            forgotten_password_email_from: config.forgotten_password_email_from,
        }
    }
}

/// Message to send
#[derive(Debug, Default, Clone)]
pub struct Message {
    pub from: String,
    pub to_list: Vec<String>,
    pub subject: String,
    pub text_body: Option<String>,
    pub html_body: Option<String>,
}

/// Implementation of `EmailService`
#[derive(Clone)]
pub struct Email {
    config: EmailConfig,
}

impl Email {
    /// Init new email with SMTP config
    pub fn init(config: EmailConfig) -> Self {
        Self { config }
    }

    /// Send an email
    fn send(&self, message: Message) -> ApiResult<()> {
        println!("Send message: {message:?}");
        Ok(())
    }
}

impl EmailService for Email {
    fn forgotten_password(&self, request: ForgottenPasswordRequest, token: &str) -> ApiResult<()> {
        let message = ForgottenPassword {
            app_name: "Backend Clean Architecture with Rust".to_owned(), // TODO: Add appplication name
            base_url: self.config.forgotten_password_base_url.clone(),
            token: token.to_string(),
            email_from: self.config.forgotten_password_email_from.clone(),
            email_to: request.email,
        };
        self.send(message.try_into()?)
    }
}
