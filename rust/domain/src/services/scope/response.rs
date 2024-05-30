//! Scope services responses

use crate::entities::scope::ScopeId;
use crate::use_cases::scope::response::{DeleteScopeUseCaseResponse, GetScopesUseCaseResponse, ScopeUseCaseResponse};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Create scope service response
pub type CreateScopeServiceResponse = ();

/// Delete scope service response
pub struct DeleteScopeServiceResponse {
    pub deleted: u64,
}

impl From<DeleteScopeUseCaseResponse> for DeleteScopeServiceResponse {
    fn from(response: DeleteScopeUseCaseResponse) -> Self {
        Self {
            deleted: response.deleted,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeServiceResponse {
    pub id: ScopeId,
    pub created_at: DateTime<Utc>,
}

impl From<ScopeServiceResponse> for ScopeUseCaseResponse {
    fn from(response: ScopeServiceResponse) -> Self {
        Self {
            id: response.id,
            created_at: response.created_at,
        }
    }
}

/// Get scopes service response
pub struct GetScopesServiceResponse {
    pub scopes: Vec<ScopeServiceResponse>,
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
