//! Scope repositories requests

use crate::entities::scope::ScopeId;
use crate::services::scope::request::{CreateScopeServiceRequest, DeleteScopeServiceRequest};
use serde::Deserialize;

/// Create a new scope request
#[derive(Debug, Clone, Deserialize)]
pub struct CreateScopeRepositoryRequest {
    pub id: ScopeId,
}

impl From<CreateScopeServiceRequest> for CreateScopeRepositoryRequest {
    fn from(req: CreateScopeServiceRequest) -> Self {
        Self { id: req.id }
    }
}

/// Delete a scope request
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteScopeRepositoryRequest {
    pub id: ScopeId,
}

impl From<DeleteScopeServiceRequest> for DeleteScopeRepositoryRequest {
    fn from(req: DeleteScopeServiceRequest) -> Self {
        Self { id: req.id }
    }
}
