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

    /// Server URL
    pub server_url: String,
    /// Server port
    pub server_port: String,
    /// Server requests timeout (in second)
    pub request_timeout: u64,

    /// JWT secret key
    pub jwt_secret_key: String,
    /// JWT lifetime
    pub jwt_lifetime: i64,

    /// CORS Allow Origin Headers (URLs delimited by a comma)
    pub cors_allow_origin: String,

    /// Basic Auth username
    pub basic_auth_username: String,
    /// Basic Auth password
    pub basic_auth_password: String,
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
