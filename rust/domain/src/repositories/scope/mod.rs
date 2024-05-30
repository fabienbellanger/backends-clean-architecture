//! Scope repository

pub mod request;
pub mod response;

use crate::repositories::scope::request::{CreateScopeRepositoryRequest, DeleteScopeRepositoryRequest};
use crate::repositories::scope::response::{
    CreateScopeRepositoryResponse, DeleteScopeRepositoryResponse, GetScopesRepositoryResponse,
};
use async_trait::async_trait;
use clean_architecture_shared::error::ApiResult;

#[async_trait]
pub trait ScopeRepository {
    /// Create a new scope
    async fn create(&self, req: CreateScopeRepositoryRequest) -> ApiResult<CreateScopeRepositoryResponse>;

    /// Get a list of scopes
    async fn get_scopes(&self) -> ApiResult<GetScopesRepositoryResponse>;

    /// Delete a scope
    async fn delete(&self, req: DeleteScopeRepositoryRequest) -> ApiResult<DeleteScopeRepositoryResponse>;
}
