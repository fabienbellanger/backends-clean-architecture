//! Refresh token MySQL repository

use crate::database::mysql::Db;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use clean_architecture_domain::entities::refresh_token::RefreshToken;
use clean_architecture_domain::ports::repositories::refresh_token::RefreshTokenRepository;
use clean_architecture_domain::ports::requests::refresh_token::{RefreshTokenId, RefreshTokenRequest};
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
    #[instrument(skip(self))]
    async fn create(&self, request: RefreshTokenRequest) -> ApiResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO refresh_tokens (refresh_token, user_id, access_token, expired_at)
                VALUES (?, ?, ?, ?)
            "#,
            request.refresh_token,
            request.user_id,
            request.access_token,
            request.expired_at,
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn get(&self, request: RefreshTokenId) -> ApiResult<RefreshToken> {
        let result = sqlx::query!(
            r#"
                SELECT refresh_token, user_id, access_token, expired_at
                FROM refresh_tokens
                WHERE refresh_token = ?
            "#,
            request.refresh_token,
        )
        .fetch_optional(self.db.pool.clone().as_ref())
        .await?;

        match result {
            None => Err(api_error!(ApiErrorCode::Unauthorized)),
            Some(record) => Ok(RefreshToken {
                refresh_token: Uuid::from_str(&record.refresh_token)
                    .map_err(|err| api_error!(ApiErrorCode::InternalError, "error when convert string to Uuid", err))?,
                user_id: Uuid::from_str(&record.user_id)
                    .map_err(|err| api_error!(ApiErrorCode::InternalError, "error when convert string to Uuid", err))?,
                access_token: record.access_token,
                expired_at: DateTime::from_naive_utc_and_offset(record.expired_at, Utc),
            }),
        }
    }

    #[instrument(skip(self))]
    async fn delete(&self, request: RefreshTokenId) -> ApiResult<()> {
        let _result = sqlx::query!(
            r#"
                DELETE FROM refresh_tokens
                WHERE refresh_token = ?
            "#,
            request.refresh_token
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        // Ok(result.rows_affected())
        Ok(())
    }
}
