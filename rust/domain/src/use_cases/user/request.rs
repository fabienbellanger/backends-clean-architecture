//! User use cases requests

use crate::entities::scope::ScopeId;
use crate::entities::user::UserId;
use serde::Deserialize;
use validator::Validate;

/// Get user scopes use case request
#[derive(Debug, Clone, Validate, Deserialize)]
pub struct GetUserScopesUseCaseRequest {
    pub user_id: UserId,
}

/// Add user scope use case request
#[derive(Debug, Clone, Validate, Deserialize)]
pub struct AddUserScopeUseCaseRequest {
    pub user_id: UserId,
    pub scope_id: ScopeId,
}

/// Remove user scope use case request
#[derive(Debug, Clone, Validate, Deserialize)]
pub struct RemoveUserScopeUseCaseRequest {
    pub user_id: UserId,
    pub scope_id: ScopeId,
}
