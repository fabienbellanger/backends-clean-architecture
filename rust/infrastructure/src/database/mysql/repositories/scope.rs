//! Scope MySQL repository

use crate::database::mysql::models::scope::ScopeModel;
use crate::database::mysql::Db;
use async_trait::async_trait;
use chrono::Utc;
use clean_architecture_domain::entities::scope::Scope;
use clean_architecture_domain::ports::repositories::scope::ScopeRepository;
use clean_architecture_domain::ports::requests::scope::{CreateRequest, DeleteRequest};
use clean_architecture_shared::error::ApiResult;
use std::sync::Arc;

/// Scope MySQL repository
#[derive(Debug, Clone)]
pub struct ScopeMysqlRepository {
    db: Arc<Db>,
}

impl ScopeMysqlRepository {
    /// Create a new repository
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl ScopeRepository for ScopeMysqlRepository {
    #[instrument(skip(self), name = "scope_repository_create")]
    async fn create(&self, req: CreateRequest) -> ApiResult<()> {
        sqlx::query!(
            r#"
                INSERT INTO scopes (id, created_at)
                VALUES (?, ?)
            "#,
            req.id,
            Utc::now(),
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(())
    }

    #[instrument(skip(self), name = "scope_repository_get_scopes")]
    async fn get_scopes(&self) -> ApiResult<Vec<Scope>> {
        let scopes = sqlx::query_as!(
            ScopeModel,
            r#"
                SELECT
                    id,
                    created_at,
                    deleted_at
                FROM scopes 
                WHERE scopes.deleted_at IS NULL
            "#
        )
        .fetch_all(self.db.pool.clone().as_ref())
        .await?;

        Ok(scopes.into_iter().map(|s| s.into()).collect())
    }

    #[instrument(skip(self), name = "scope_repository_delete")]
    async fn delete(&self, req: DeleteRequest) -> ApiResult<u64> {
        let result = sqlx::query!(
            r#"
                UPDATE scopes
                SET deleted_at = ?
                WHERE id = ?
            "#,
            Utc::now(),
            req.id
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(result.rows_affected())
    }
}
