//! User repositories responses

use crate::entities::scope::ScopeId;
use serde::Deserialize;

/// Get user scopes repository response
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserScopesRepositoryResponse {
    pub scopes: Vec<UserScopeRepositoryResponse>,
}

/// Get user scope repository response
#[derive(Debug, Clone, Deserialize)]
pub struct UserScopeRepositoryResponse {
    pub id: ScopeId,
}

/// Add user scope repository response
#[derive(Debug, PartialEq, Eq)]
pub struct AddUserScopeRepositoryResponse {
    pub created: u64,
}

/// Remove user scope repository response
#[derive(Debug, PartialEq, Eq)]
pub struct RemoveUserScopeRepositoryResponse {
    pub deleted: u64,
}
