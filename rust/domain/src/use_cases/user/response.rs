//! User use cases responses

use crate::entities::scope::ScopeId;
use serde::Deserialize;
use validator::Validate;

/// Get user scopes use case response
#[derive(Debug, Clone, Validate, Deserialize)]
pub struct GetUserScopesUseCaseResponse {
    pub scopes: Vec<ScopeId>,
}

/// Add user scope use case response
#[derive(Debug, PartialEq, Eq)]
pub struct AddUserScopeUseCaseResponse {
    pub created: u64,
}

/// Remove user scope use case response
#[derive(Debug, PartialEq, Eq)]
pub struct RemoveUserScopeUseCaseResponse {
    pub deleted: u64,
}
