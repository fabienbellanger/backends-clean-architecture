//! Email

mod forgotten_password;

use crate::config::Config;
use crate::email::forgotten_password::ForgottenPassword;
use crate::APP_NAME;
use clean_architecture_domain::requests::user::ForgottenPasswordRequest;
use clean_architecture_domain::services::email::EmailService;
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
use lettre::message::{header, MultiPart, SinglePart};
use lettre::{SmtpTransport, Transport};
use std::time::Duration;

#[derive(Debug, Default, Clone)]
pub struct EmailConfig {
    host: String,
    port: u16,
    timeout: u64,

    /// Forgotten password config
    forgotten_password_base_url: String,
    forgotten_password_email_from: String,
}

impl From<Config> for EmailConfig {
    fn from(config: Config) -> Self {
        EmailConfig {
            host: config.smtp_host,
            port: config.smtp_port,
            timeout: config.smtp_timeout,
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
    /// New email with SMTP config
    pub fn new(config: EmailConfig) -> Self {
        Self { config }
    }

    /// Initialize email sending
    fn init(&self) -> SmtpTransport {
        let host = &self.config.host;
        let port = self.config.port;
        let timeout = match self.config.timeout {
            0 => None,
            t => Some(Duration::from_secs(t)),
        };

        SmtpTransport::builder_dangerous(host)
            .port(port)
            .timeout(timeout)
            .build()
    }

    /// Send an email
    fn send(&self, message: Message) -> ApiResult<()> {
        let mailer = self.init();

        let mut email_builder =
            lettre::Message::builder()
                .subject(message.subject)
                .from(message.from.parse().map_err(|_| {
                    api_error!(
                        ApiErrorCode::InternalError,
                        "cannot send password reset email because: invalid from email".to_string()
                    )
                })?);

        // Add destination emails
        for to in message.to_list {
            email_builder = email_builder.to(to.parse().map_err(|_| {
                api_error!(
                    ApiErrorCode::InternalError,
                    "cannot send password reset email because: invalid to email".to_string()
                )
            })?)
        }

        let mut multipart = MultiPart::alternative().build();
        if let Some(text) = message.text_body {
            multipart = multipart.singlepart(SinglePart::builder().header(header::ContentType::TEXT_PLAIN).body(text));
        }
        if let Some(html) = message.html_body {
            multipart = multipart.singlepart(SinglePart::builder().header(header::ContentType::TEXT_HTML).body(html));
        }
        let email = email_builder.multipart(multipart).map_err(|err| {
            api_error!(
                ApiErrorCode::InternalError,
                format!("cannot send password reset email because: {err}")
            )
        })?;

        mailer.send(&email).map_err(|err| {
            api_error!(
                ApiErrorCode::InternalError,
                format!("SMTP Error when sending password reset email: {err}")
            )
        })?;

        Ok(())
    }
}

impl EmailService for Email {
    fn forgotten_password(&self, request: ForgottenPasswordRequest, token: &str) -> ApiResult<()> {
        let message = ForgottenPassword {
            app_name: APP_NAME.to_owned(),
            base_url: self.config.forgotten_password_base_url.clone(),
            token: token.to_string(),
            email_from: self.config.forgotten_password_email_from.clone(),
            email_to: request.email,
        };
        self.send(message.try_into()?)
    }
}
