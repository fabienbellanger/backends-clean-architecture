//! Scope services responses

use crate::entities::scope::ScopeId;
use crate::repositories::scope::response::{
    DeleteScopeRepositoryResponse, GetScopesRepositoryResponse, ScopeRepositoryResponse,
};
use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// Create scope service response
pub type CreateScopeServiceResponse = ();

/// Delete scope service response
pub struct DeleteScopeServiceResponse {
    pub deleted: u64,
}

impl From<DeleteScopeRepositoryResponse> for DeleteScopeServiceResponse {
    fn from(response: DeleteScopeRepositoryResponse) -> Self {
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

impl From<ScopeRepositoryResponse> for ScopeServiceResponse {
    fn from(response: ScopeRepositoryResponse) -> Self {
        Self {
            id: response.id,
            created_at: Utc.from_utc_datetime(&response.created_at),
        }
    }
}

/// Get scopes service response
pub struct GetScopesServiceResponse {
    pub scopes: Vec<ScopeServiceResponse>,
}

impl From<GetScopesRepositoryResponse> for GetScopesServiceResponse {
    fn from(response: GetScopesRepositoryResponse) -> Self {
        Self {
            scopes: response
                .scopes
                .into_iter()
                .map(|scope| ScopeServiceResponse {
                    id: scope.id,
                    created_at: Utc.from_utc_datetime(&scope.created_at),
                })
                .collect(),
        }
    }
}
