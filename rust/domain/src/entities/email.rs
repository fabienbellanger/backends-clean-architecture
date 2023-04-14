//! Email entity

use clean_architecture_shared::error::ApiResult;

/// Message to send
#[derive(Default)]
pub struct Message {
    pub from: String,
    pub to_list: Vec<String>,
    pub subject: String,
    pub text_body: Option<String>,
    pub html_body: Option<String>,
}

#[derive(Debug, Default, Clone)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub timeout: u64,
    pub username: Option<String>,
    pub password: Option<String>,
}

pub trait Email {
    fn send(&self, config: &SmtpConfig, message: &Message) -> ApiResult<()>;
}
