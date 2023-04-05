pub mod config;
pub mod mysql;

use self::config::Config;
use async_trait::async_trait;
use clean_architecture_shared::error::ApiResult;

/// Initialize configuration
fn init_config() -> ApiResult<Config> {
    Config::from_env()
}

#[async_trait]
pub trait GenericDb {
    type Db;

    async fn new() -> ApiResult<Self::Db>;
    async fn from_config(config: &Config) -> ApiResult<Self::Db>;
}
