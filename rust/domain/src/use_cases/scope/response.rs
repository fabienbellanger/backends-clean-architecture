//! Scope use cases responses

use crate::entities::scope::ScopeId;
use crate::services::scope::response::{DeleteScopeServiceResponse, GetScopesServiceResponse};
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

impl From<GetScopesServiceResponse> for GetScopesUseCaseResponse {
    fn from(response: GetScopesServiceResponse) -> Self {
        Self {
            scopes: response
                .scopes
                .into_iter()
                .map(|scope| ScopeUseCaseResponse {
                    id: scope.id,
                    created_at: scope.created_at,
                })
                .collect(),
        }
    }
}
