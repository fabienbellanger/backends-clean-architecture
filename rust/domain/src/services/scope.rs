//! Scope services

use crate::repositories::scope::ScopeRepository;
use crate::requests::scope::{CreateRequest, DeleteRequest};
use crate::responses::scope::ScopeResponse;
use clean_architecture_shared::error::ApiResult;

#[derive(Clone)]
pub struct ScopeService<S: ScopeRepository> {
    scope_repository: S,
}

impl<S: ScopeRepository> ScopeService<S> {
    /// Create a new service
    pub fn new(scope_repository: S) -> Self {
        Self { scope_repository }
    }

    /// Create a new scope
    #[instrument(skip_all, name = "scope_service_create")]
    pub async fn create(&self, req: CreateRequest) -> ApiResult<()> {
        self.scope_repository.create(req).await
    }

    /// Get a list of scopes
    #[instrument(skip_all, name = "scope_service_get_scopes")]
    pub async fn get_scopes(&self) -> ApiResult<Vec<ScopeResponse>> {
        self.scope_repository
            .get_scopes()
            .await
            .map(|scopes| scopes.into_iter().map(|scope| scope.into()).collect())
    }

    /// Delete a scope
    #[instrument(skip_all, name = "scope_service_delete")]
    pub async fn delete(&self, req: DeleteRequest) -> ApiResult<u64> {
        self.scope_repository.delete(req).await
    }
}
