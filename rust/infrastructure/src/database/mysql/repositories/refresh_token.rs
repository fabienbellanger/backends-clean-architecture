//! Refresh token MySQL repository

use crate::database::mysql::Db;
use async_trait::async_trait;
use clean_architecture_domain::ports::repositories::refresh_token::RefreshTokenRepository;
use clean_architecture_domain::ports::requests::refresh_token::RefreshTokenRequest;
use clean_architecture_shared::error::ApiResult;
use std::sync::Arc;

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
}
