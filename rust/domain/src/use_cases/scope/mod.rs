//! Scope use case

pub mod request;
pub mod response;

use crate::repositories::scope::ScopeRepository;
use crate::services::scope::ScopeService;
use crate::use_cases::scope::request::{CreateScopeUseCaseRequest, DeleteScopeUseCaseRequest};
use crate::use_cases::scope::response::{
    CreateScopeUseCaseResponse, DeleteScopeUseCaseResponse, GetScopesUseCaseResponse,
};
use clean_architecture_shared::error::ApiResult;
use clean_architecture_shared::validation::validate_request_data;

/// Create new scope use case
#[derive(Clone)]
pub struct ScopeUseCase<S: ScopeRepository> {
    scope_service: ScopeService<S>,
}

impl<S: ScopeRepository> ScopeUseCase<S> {
    /// Create a new use case
    pub fn new(scope_service: ScopeService<S>) -> Self {
        Self { scope_service }
    }

    /// Create a new scope
    #[instrument(skip(self), name = "scope_use_case_create")]
    pub async fn create(&self, req: CreateScopeUseCaseRequest) -> ApiResult<CreateScopeUseCaseResponse> {
        validate_request_data(&req)?;

        self.scope_service.create(req.into()).await
    }

    /// Get a list of scopes
    #[instrument(skip(self), name = "scope_use_case_get_scopes")]
    pub async fn get_scopes(&self) -> ApiResult<GetScopesUseCaseResponse> {
        Ok(self.scope_service.get_scopes().await?.into())
    }

    /// Delete a scope
    #[instrument(skip(self), name = "scope_use_case_delete")]
    pub async fn delete(&self, req: DeleteScopeUseCaseRequest) -> ApiResult<DeleteScopeUseCaseResponse> {
        validate_request_data(&req)?;

        Ok(self.scope_service.delete(req.into()).await?.into())
    }
}
