//! Configuration module

use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
use serde::Deserialize;

/// Represents configuration structure
#[derive(Deserialize, Clone, Debug, Default)]
pub struct Config {
    /// Environment: `development` or `production`
    pub environment: String,

    /// Rust logs
    pub rust_log: String,

    /// Path of log files
    pub logs_path: String,
    /// Log file name
    pub logs_file: String,

    /// Database URL (Ex.: mysql://root:root@127.0.0.1:3306/rust_clean_architecture)
    pub database_url: String,
    /// Database auto migration enabled
    pub database_auto_migration: bool,
    /// Database maximum connections (in second)
    pub database_max_connections: u32,
    /// Database minimum connections (in second)
    pub database_min_connections: u32,
    /// Database maximum lifetime (in second)
    pub database_max_lifetime: u64,
    /// Database connection timeout (in second)
    pub database_connect_timeout: u64,
    /// Database connection timeout (in second)
    pub database_idle_timeout: u64,

    /// Server URL
    pub server_url: String,
    /// Server port
    pub server_port: String,
    /// Server requests timeout (in second)
    pub request_timeout: u64,

    /// JWT algorithm
    pub jwt_algorithm: String,
    /// JWT secret key
    pub jwt_secret_key: Option<String>,
    /// JWT private key
    pub jwt_private_key: Option<String>,
    /// JWT public key
    pub jwt_public_key: Option<String>,
    /// JWT access token lifetime
    pub jwt_access_lifetime: i64,
    /// JWT refresh token lifetime
    pub jwt_refresh_lifetime: i64,

    /// CORS Allow Origin Headers (URLs delimited by a comma)
    pub cors_allow_origin: String,

    /// Basic Auth username
    pub basic_auth_username: String,
    /// Basic Auth password
    pub basic_auth_password: String,

    /// SMTP host
    pub smtp_host: String,
    /// SMTP port
    pub smtp_port: u16,
    /// SMTP timeout (in second)
    pub smtp_timeout: u64,
    /// SMTP username
    pub smtp_username: String,
    /// SMTP password
    pub smtp_password: String,

    /// Forgotten password expiration duration (in hour)
    pub forgotten_password_expiration_duration: i64,
    /// Forgotten password base URL for link (Ex.: http://localhost)
    pub forgotten_password_base_url: String,
    /// Forgotten password email from
    pub forgotten_password_email_from: String,

    /// Prometheus metics enabled
    pub prometheus_metrics_enabled: bool,
}

impl Config {
    /// from_env loads configuration from environment variables
    pub fn from_env() -> ApiResult<Config> {
        dotenvy::dotenv().ok();

        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?
            .try_deserialize()
            .map_err(|err| api_error!(ApiErrorCode::InternalError, err))
    }
}
