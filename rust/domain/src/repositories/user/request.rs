//! User repositories requests

use crate::entities::scope::ScopeId;
use crate::entities::user::UserId;
use crate::services::user::request::{
    AddUserScopeServiceRequest, GetUserScopesServiceRequest, RemoveUserScopeServiceRequest,
};
use serde::Deserialize;

/// Get user scopes repository request
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserScopesRepositoryRequest {
    pub user_id: UserId,
}

impl From<GetUserScopesServiceRequest> for GetUserScopesRepositoryRequest {
    fn from(req: GetUserScopesServiceRequest) -> Self {
        Self { user_id: req.user_id }
    }
}

/// Add user scope repository request
#[derive(Debug, Clone, Deserialize)]
pub struct AddUserScopeRepositoryRequest {
    pub user_id: UserId,
    pub scope_id: ScopeId,
}

impl From<AddUserScopeServiceRequest> for AddUserScopeRepositoryRequest {
    fn from(req: AddUserScopeServiceRequest) -> Self {
        Self {
            user_id: req.user_id,
            scope_id: req.scope_id,
        }
    }
}

/// Remove user scope repository request
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveUserScopeRepositoryRequest {
    pub user_id: UserId,
    pub scope_id: ScopeId,
}

impl From<RemoveUserScopeServiceRequest> for RemoveUserScopeRepositoryRequest {
    fn from(req: RemoveUserScopeServiceRequest) -> Self {
        Self {
            user_id: req.user_id,
            scope_id: req.scope_id,
        }
    }
}
