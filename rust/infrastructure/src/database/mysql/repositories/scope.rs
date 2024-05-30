//! Scope MySQL repository

use crate::database::mysql::Db;
use async_trait::async_trait;
use chrono::Utc;
use clean_architecture_domain::repositories::scope::request::{
    CreateScopeRepositoryRequest, DeleteScopeRepositoryRequest,
};
use clean_architecture_domain::repositories::scope::response::{
    DeleteScopeRepositoryResponse, GetScopesRepositoryResponse, ScopeRepositoryResponse,
};
use clean_architecture_domain::repositories::scope::ScopeRepository;
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
    async fn create(&self, req: CreateScopeRepositoryRequest) -> ApiResult<()> {
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
    async fn get_scopes(&self) -> ApiResult<GetScopesRepositoryResponse> {
        let scopes = sqlx::query_as!(
            ScopeRepositoryResponse,
            r#"
                SELECT
                    id,
                    created_at
                FROM scopes 
                WHERE scopes.deleted_at IS NULL
            "#
        )
        .fetch_all(self.db.pool.clone().as_ref())
        .await?;

        Ok(GetScopesRepositoryResponse { scopes })
    }

    #[instrument(skip(self), name = "scope_repository_delete")]
    async fn delete(&self, req: DeleteScopeRepositoryRequest) -> ApiResult<DeleteScopeRepositoryResponse> {
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

        Ok(DeleteScopeRepositoryResponse {
            deleted: result.rows_affected(),
        })
    }
}
