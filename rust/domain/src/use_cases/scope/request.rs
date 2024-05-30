//! Scope use cases requests

use crate::entities::scope::ScopeId;
use serde::Deserialize;
use validator::Validate;

/// Create a new scope request
#[derive(Debug, Clone, Validate, Deserialize)]
pub struct CreateScopeUseCaseRequest {
    #[validate(length(min = 4))]
    pub id: ScopeId,
}

/// Delete a scope request
#[derive(Debug, Clone, Validate, Deserialize)]
pub struct DeleteScopeUseCaseRequest {
    #[validate(length(min = 4))]
    pub id: ScopeId,
}
