//! Email entity

/// Represents SMTP configuration
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub timeout: u64,
}

/// Message to send
#[derive(Default)]
pub struct Message {
    pub from: String,
    pub to_list: Vec<String>,
    pub subject: String,
    pub text_body: Option<String>,
    pub html_body: Option<String>,
}

/// Represents an email sent
pub struct Email {
    pub smtp_config: SmtpConfig,
    pub message: Message,
}

impl Email {
    // /// Creates a new `Email` from SMTP configuration
    // pub fn init(config: &SmtpConfig) -> Self {
    //     Self {
    //         smtp_config: config,
    //         ..default::Default,
    //     }
    // }
}
