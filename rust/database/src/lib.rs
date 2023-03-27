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
///
/// ```ignore
/// use clean_architecture_database::{
///     init_mysql_pool, mysql::repositories::user::UserMysqlRepository,
/// };
/// use clean_architecture_shared::error::ApiResult;
///
/// #[tokio::main]
/// async fn main() -> ApiResult<()> {
///     let pool = init_mysql_pool().await?;
///     let _user_repository = UserMysqlRepository::new(&pool);
///
///     Ok(())
/// }
/// ```
pub async fn init_mysql_pool() -> ApiResult<Pool<MySql>> {
    databases::init_mysql(&init_config()?).await
}
