//! MySQL module

pub mod models;
pub mod repositories;

use crate::{config::Config, database::GenericDb, init_config};
use async_trait::async_trait;
use clean_architecture_shared::{
    api_error,
    error::{ApiError, ApiErrorCode, ApiResult},
};
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::{sync::Arc, time::Duration};

#[derive(Debug, Clone)]
pub struct Db {
    pub pool: Arc<Pool<MySql>>,
}

#[async_trait]
impl GenericDb for Db {
    type Db = Db;

    async fn new() -> ApiResult<Self::Db> {
        let settings = &init_config()?;
        Self::from_config(settings).await
    }

    async fn from_config(settings: &Config) -> ApiResult<Self::Db> {
        let url = &settings.database_url;
        let max_connections = settings.database_max_connections;
        let min_connections = settings.database_min_connections;
        let max_lifetime = settings.database_max_lifetime;
        let connect_timeout = settings.database_connect_timeout;
        let idle_timeout = settings.database_idle_timeout;

        let pool = MySqlPoolOptions::new()
            .max_connections(max_connections)
            .min_connections(min_connections)
            .max_lifetime(Some(Duration::from_secs(max_lifetime)))
            .acquire_timeout(Duration::from_secs(connect_timeout))
            .idle_timeout(Duration::from_secs(idle_timeout))
            .test_before_acquire(true)
            .connect(url)
            .await
            .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?;

        if settings.database_auto_migration {
            info!("Run database migrations");
            sqlx::migrate!("./migrations")
                .run(&pool)
                .await
                .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?
        }

        Ok(Self {
            pool: Arc::new(pool),
        })
    }
}
