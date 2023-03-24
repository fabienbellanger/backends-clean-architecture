pub(crate) mod config;
pub(crate) mod databases;
pub mod mysql;

#[macro_use]
extern crate tracing;

use crate::config::Config;
use clean_architecture_shared::error::ApiResult;
use sqlx::{MySql, Pool};

/// Initialize configuration
fn init_config() -> ApiResult<Config> {
    Config::from_env()
}

/// Initialize MySQL connection pool
pub async fn init_mysql_pool() -> ApiResult<Pool<MySql>> {
    databases::init_mysql(&init_config()?).await
}
