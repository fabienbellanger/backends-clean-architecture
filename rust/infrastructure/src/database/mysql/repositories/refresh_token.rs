//! Refresh token MySQL repository

use crate::database::mysql::Db;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use clean_architecture_domain::entities::refresh_token::RefreshToken;
use clean_architecture_domain::repositories::refresh_token::request::{
    CreateRefreshTokenRepositoryRequest, GetRefreshTokenRepositoryRequest, RemoveRefreshTokenRepositoryRequest,
};
use clean_architecture_domain::repositories::refresh_token::response::{
    CreateRefreshTokenRepositoryResponse, GetRefreshTokenRepositoryResponse, RemoveRefreshTokenRepositoryResponse,
};
use clean_architecture_domain::repositories::refresh_token::RefreshTokenRepository;
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

/// Password Reset MySQL repository
#[derive(Debug, Clone)]
pub struct RefreshTokenMysqlRepository {
    db: Arc<Db>,
}

impl RefreshTokenMysqlRepository {
    /// Create a new repository
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl RefreshTokenRepository for RefreshTokenMysqlRepository {
    #[instrument(skip(self), name = "refresh_token_repository_create")]
    async fn create(
        &self,
        request: CreateRefreshTokenRepositoryRequest,
    ) -> ApiResult<CreateRefreshTokenRepositoryResponse> {
        sqlx::query!(
            r#"
                INSERT INTO refresh_tokens (refresh_token, user_id, access_token, expired_at)
                VALUES (?, ?, ?, ?)
            "#,
            request.token.refresh_token.to_string(),
            request.token.user_id.to_string(),
            request.token.access_token,
            request.token.expired_at,
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(())
    }

    #[instrument(skip(self), name = "refresh_token_repository_get")]
    async fn get(&self, request: GetRefreshTokenRepositoryRequest) -> ApiResult<GetRefreshTokenRepositoryResponse> {
        let result = sqlx::query!(
            r#"
                SELECT refresh_token, user_id, access_token, expired_at
                FROM refresh_tokens
                WHERE refresh_token = ?
                LIMIT 1
            "#,
            request.token.to_string(),
        )
        .fetch_optional(self.db.pool.clone().as_ref())
        .await?;

        match result {
            None => Err(api_error!(ApiErrorCode::Unauthorized)),
            Some(record) => Ok(GetRefreshTokenRepositoryResponse {
                token: RefreshToken {
                    refresh_token: Uuid::from_str(&record.refresh_token).map_err(|err| {
                        api_error!(ApiErrorCode::InternalError, "error when convert string to Uuid", err)
                    })?,
                    user_id: Uuid::from_str(&record.user_id).map_err(|err| {
                        api_error!(ApiErrorCode::InternalError, "error when convert string to Uuid", err)
                    })?,
                    access_token: record.access_token,
                    expired_at: DateTime::from_naive_utc_and_offset(record.expired_at, Utc),
                },
            }),
        }
    }

    #[instrument(skip(self), name = "refresh_token_repository_delete")]
    async fn delete(
        &self,
        request: RemoveRefreshTokenRepositoryRequest,
    ) -> ApiResult<RemoveRefreshTokenRepositoryResponse> {
        let result = sqlx::query!(
            r#"
                DELETE FROM refresh_tokens
                WHERE refresh_token = ?
            "#,
            request.token.to_string(),
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(RemoveRefreshTokenRepositoryResponse {
            deleted: result.rows_affected(),
        })
    }
}
