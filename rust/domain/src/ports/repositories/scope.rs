//! Scope repository

use crate::entities::scope::Scope;
use crate::ports::requests::scope::{CreateRequest, DeleteRequest};
use async_trait::async_trait;
use clean_architecture_shared::error::ApiResult;

#[async_trait]
pub trait ScopeRepository {
    /// Create a new scope
    async fn create(&self, req: CreateRequest) -> ApiResult<()>;

    /// Get a list of scopes
    async fn get_scopes(&self) -> ApiResult<Vec<Scope>>;

    /// Delete a scope
    async fn delete(&self, req: DeleteRequest) -> ApiResult<u64>;
}
