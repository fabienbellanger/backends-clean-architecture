//! User MySQL repository module

use crate::mysql::models::user::UserModel;
use async_trait::async_trait;
use clean_architecture_domain::{
    entities::user::User,
    ports::{
        repositories::user::UserRepository,
        requests::user::{GetUserRequest, LoginRequest},
    },
};
use clean_architecture_shared::{
    api_error,
    error::{ApiError, ApiErrorCode, ApiResult},
};
use sha2::{Digest, Sha512};
use sqlx::mysql::MySqlPool;
use tracing::instrument;

#[derive(Debug)]
pub struct UserMysqlRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> UserMysqlRepository<'a> {
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'a> UserRepository for UserMysqlRepository<'a> {
    #[instrument(skip(self))]
    async fn get_users(&self) -> ApiResult<Vec<User>> {
        let users = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE deleted_at IS NULL")
            .fetch_all(self.pool)
            .await
            .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?;

        Ok(users
            .into_iter()
            .filter_map(|u| u.try_into().ok())
            .collect())
    }

    #[instrument(skip(self))]
    async fn get_user(&self, request: GetUserRequest) -> ApiResult<User> {
        let user = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE id = ?", request.id)
            .fetch_one(self.pool)
            .await
            .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?;

        user.try_into()
    }

    #[instrument(skip(self))]
    async fn login(&self, request: LoginRequest) -> ApiResult<User> {
        let hashed_password = format!("{:x}", Sha512::digest(request.password.as_bytes()));
        let user = sqlx::query_as!(
            UserModel,
            "SELECT * FROM users WHERE email = ? AND password = ?",
            request.email,
            hashed_password
        )
        .fetch_one(self.pool)
        .await
        .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?;

        user.try_into()
    }
}
