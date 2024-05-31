//! User use cases responses

use crate::entities::refresh_token::RefreshTokenId;
use crate::entities::scope::ScopeId;
use crate::services::user::response::{
    AddUserScopeServiceResponse, GetRefreshTokenServiceResponse, GetUserScopesServiceResponse,
    RemoveUserScopeServiceResponse,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Get user scopes use case response
#[derive(Debug, Clone, Deserialize)]
pub struct GetUserScopesUseCaseResponse {
    pub scopes: Vec<ScopeId>,
}

impl From<GetUserScopesServiceResponse> for GetUserScopesUseCaseResponse {
    fn from(response: GetUserScopesServiceResponse) -> Self {
        Self {
            scopes: response.scopes,
        }
    }
}

/// Add user scope use case response
#[derive(Debug, PartialEq, Eq)]
pub struct AddUserScopeUseCaseResponse {
    pub created: u64,
}

impl From<AddUserScopeServiceResponse> for AddUserScopeUseCaseResponse {
    fn from(value: AddUserScopeServiceResponse) -> Self {
        Self { created: value.created }
    }
}

/// Remove user scope use case response
#[derive(Debug, PartialEq, Eq)]
pub struct RemoveUserScopeUseCaseResponse {
    pub deleted: u64,
}

impl From<RemoveUserScopeServiceResponse> for RemoveUserScopeUseCaseResponse {
    fn from(value: RemoveUserScopeServiceResponse) -> Self {
        Self { deleted: value.deleted }
    }
}

/// Get refresh token use case response
#[derive(Debug, PartialEq, Eq, Serialize)]
pub struct GetRefreshTokenUseCaseResponse {
    pub access_token: String,
    pub access_token_expired_at: DateTime<Utc>,
    pub refresh_token: RefreshTokenId,
    pub refresh_token_expired_at: DateTime<Utc>,
}

impl From<GetRefreshTokenServiceResponse> for GetRefreshTokenUseCaseResponse {
    fn from(value: GetRefreshTokenServiceResponse) -> Self {
        Self {
            access_token: value.access_token,
            access_token_expired_at: value.access_token_expired_at,
            refresh_token: value.refresh_token,
            refresh_token_expired_at: value.refresh_token_expired_at,
        }
    }
}
