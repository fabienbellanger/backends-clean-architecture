pub(crate) mod config;
pub mod database;
pub mod mysql;

#[macro_use]
extern crate tracing;

use crate::config::Config;
use clean_architecture_shared::error::ApiResult;

/// Initialize configuration
fn init_config() -> ApiResult<Config> {
    Config::from_env()
}
