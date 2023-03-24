//! Configuration module

use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
use serde::Deserialize;

/// Represents configuration structure
#[derive(Deserialize, Clone, Debug, Default)]
pub struct Config {
    /// Environment: `developement` or `production`
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
