//! User use cases responses

use crate::entities::scope::ScopeId;
use crate::services::user::response::{
    AddUserScopeServiceResponse, GetUserScopesServiceResponse, RemoveUserScopeServiceResponse,
};
use serde::Deserialize;

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
