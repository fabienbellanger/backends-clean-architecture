//! Scope services requests

use crate::entities::scope::ScopeId;
use crate::use_cases::scope::request::{CreateScopeUseCaseRequest, DeleteScopeUseCaseRequest};
use serde::Deserialize;

/// Create a new scope request
#[derive(Debug, Clone, Deserialize)]
pub struct CreateScopeServiceRequest {
    pub id: ScopeId,
}

impl From<CreateScopeUseCaseRequest> for CreateScopeServiceRequest {
    fn from(req: CreateScopeUseCaseRequest) -> Self {
        Self { id: req.id }
    }
}

/// Delete a scope request
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteScopeServiceRequest {
    pub id: ScopeId,
}

impl From<DeleteScopeUseCaseRequest> for DeleteScopeServiceRequest {
    fn from(req: DeleteScopeUseCaseRequest) -> Self {
        Self { id: req.id }
    }
}
