//! Scope repositories responses

use crate::entities::scope::ScopeId;
use crate::services::scope::response::{DeleteScopeServiceResponse, GetScopesServiceResponse, ScopeServiceResponse};
use chrono::{NaiveDateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// Create scope repository response
pub type CreateScopeRepositoryResponse = ();

/// Delete scope repository response
pub struct DeleteScopeRepositoryResponse {
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
pub struct ScopeRepositoryResponse {
    pub id: ScopeId,
    pub created_at: NaiveDateTime,
}

impl From<ScopeRepositoryResponse> for ScopeServiceResponse {
    fn from(response: ScopeRepositoryResponse) -> Self {
        Self {
            id: response.id,
            created_at: Utc.from_utc_datetime(&response.created_at),
        }
    }
}

/// Get scopes repository response
pub struct GetScopesRepositoryResponse {
    pub scopes: Vec<ScopeRepositoryResponse>,
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
