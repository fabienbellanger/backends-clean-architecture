//! Scope services

pub mod request;
pub mod response;

use crate::repositories::scope::ScopeRepository;
use crate::services::scope::request::{CreateScopeServiceRequest, DeleteScopeServiceRequest};
use crate::services::scope::response::{
    CreateScopeServiceResponse, DeleteScopeServiceResponse, GetScopesServiceResponse,
};
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
    pub async fn create(&self, req: CreateScopeServiceRequest) -> ApiResult<CreateScopeServiceResponse> {
        self.scope_repository.create(req.into()).await
    }

    /// Get a list of scopes
    #[instrument(skip_all, name = "scope_service_get_scopes")]
    pub async fn get_scopes(&self) -> ApiResult<GetScopesServiceResponse> {
        Ok(self.scope_repository.get_scopes().await?.into())
    }

    /// Delete a scope
    #[instrument(skip_all, name = "scope_service_delete")]
    pub async fn delete(&self, req: DeleteScopeServiceRequest) -> ApiResult<DeleteScopeServiceResponse> {
        Ok(self.scope_repository.delete(req.into()).await?.into())
    }
}
