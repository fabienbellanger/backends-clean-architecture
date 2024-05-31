//! User services requests

use crate::entities::refresh_token::RefreshTokenId;
use crate::entities::scope::ScopeId;
use crate::entities::user::UserId;
use crate::use_cases::user::request::{
    AddUserScopeUseCaseRequest, GetRefreshTokenUseCaseRequest, GetUserScopesUseCaseRequest,
    RemoveUserScopeUseCaseRequest,
};
use clean_architecture_shared::auth::Jwt;
use serde::Deserialize;

/// Get user scopes service request
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserScopesServiceRequest {
    pub user_id: UserId,
}

impl From<GetUserScopesUseCaseRequest> for GetUserScopesServiceRequest {
    fn from(req: GetUserScopesUseCaseRequest) -> Self {
        Self { user_id: req.user_id }
    }
}

/// Add user scope service request
#[derive(Debug, Clone, Deserialize)]
pub struct AddUserScopeServiceRequest {
    pub user_id: UserId,
    pub scope_id: ScopeId,
}

impl From<AddUserScopeUseCaseRequest> for AddUserScopeServiceRequest {
    fn from(req: AddUserScopeUseCaseRequest) -> Self {
        Self {
            user_id: req.user_id,
            scope_id: req.scope_id,
        }
    }
}

/// Remove user scope service request
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveUserScopeServiceRequest {
    pub user_id: UserId,
    pub scope_id: ScopeId,
}

impl From<RemoveUserScopeUseCaseRequest> for RemoveUserScopeServiceRequest {
    fn from(req: RemoveUserScopeUseCaseRequest) -> Self {
        Self {
            user_id: req.user_id,
            scope_id: req.scope_id,
        }
    }
}

/// Get refresh token service request
#[derive(Debug, Clone)]
pub struct GetRefreshTokenServiceRequest {
    pub token: RefreshTokenId,
    pub jwt: Jwt,
}

impl From<GetRefreshTokenUseCaseRequest> for GetRefreshTokenServiceRequest {
    fn from(req: GetRefreshTokenUseCaseRequest) -> Self {
        Self {
            token: req.token,
            jwt: req.jwt,
        }
    }
}
