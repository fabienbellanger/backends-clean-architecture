//! Scope repositories responses

use crate::entities::scope::ScopeId;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

/// Create scope repository response
pub type CreateScopeRepositoryResponse = ();

/// Delete scope repository response
pub struct DeleteScopeRepositoryResponse {
    pub deleted: u64,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeRepositoryResponse {
    pub id: ScopeId,
    pub created_at: NaiveDateTime,
}

/// Get scopes repository response
pub struct GetScopesRepositoryResponse {
    pub scopes: Vec<ScopeRepositoryResponse>,
}
