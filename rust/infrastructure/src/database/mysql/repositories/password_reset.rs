//! Password Reset MySQL repository

use crate::database::mysql::Db;
use async_trait::async_trait;
use chrono::Utc;
use clean_architecture_domain::ports::{
    repositories::password_reset::PasswordResetRepository,
    requests::password_reset::{DeleteRequest, GetByTokenRequest, PasswordResetRequest},
};
use clean_architecture_shared::error::ApiResult;
use std::sync::Arc;

/// Password Reset MySQL repository
#[derive(Debug, Clone)]
pub struct PasswordResetMysqlRepository {
    db: Arc<Db>,
}

impl PasswordResetMysqlRepository {
    /// Create a new repository
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl PasswordResetRepository for PasswordResetMysqlRepository {
    #[instrument(skip(self))]
    async fn create_or_update(&self, request: PasswordResetRequest) -> ApiResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO password_resets (user_id, token, expired_at)
                VALUES (?, ?, ?)
                ON DUPLICATE KEY UPDATE token = ?, expired_at = ?
            "#,
            request.user_id.to_string(),
            request.token,
            request.expired_at,
            request.token,
            request.expired_at,
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(())
    }

    #[instrument(skip(self))]
    async fn get_by_token(&self, request: GetByTokenRequest) -> ApiResult<Option<String>> {
        let result = sqlx::query!(
            r#"
                SELECT u.id AS user_id
                FROM password_resets pr
                    INNER JOIN users u ON u.id = pr.user_id AND u.deleted_at IS NULL
                WHERE pr.token = ?
                    AND pr.expired_at >= ?
            "#,
            request.token,
            Utc::now(),
        )
        .fetch_optional(self.db.pool.clone().as_ref())
        .await?;

        match result {
            Some(result) => Ok(Some(result.user_id)),
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    async fn delete(&self, request: DeleteRequest) -> ApiResult<u64> {
        let result = sqlx::query!(
            r#"
                DELETE FROM password_resets
                WHERE user_id = ?
            "#,
            request.user_id
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(result.rows_affected())
    }
}
