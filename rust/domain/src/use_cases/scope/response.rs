//! Scope use cases responses

use crate::entities::scope::ScopeId;
use crate::services::scope::response::DeleteScopeServiceResponse;
use chrono::{DateTime, Utc};

/// Create scope use case response
pub type CreateScopeUseCaseResponse = ();

/// Delete scope use case response
#[derive(Debug, PartialEq, Eq)]
pub struct DeleteScopeUseCaseResponse {
    pub deleted: u64,
}

impl From<DeleteScopeServiceResponse> for DeleteScopeUseCaseResponse {
    fn from(response: DeleteScopeServiceResponse) -> Self {
        Self {
            deleted: response.deleted,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ScopeUseCaseResponse {
    pub id: ScopeId,
    pub created_at: DateTime<Utc>,
}

/// Get scopes use case response
#[derive(Debug, PartialEq, Eq)]
pub struct GetScopesUseCaseResponse {
    pub scopes: Vec<ScopeUseCaseResponse>,
}
