//! User services responses

use crate::entities::scope::ScopeId;
use crate::repositories::user::response::{
    AddUserScopeRepositoryResponse, GetUserScopesRepositoryResponse, RemoveUserScopeRepositoryResponse,
};
use serde::Deserialize;
use validator::Validate;

/// Get user scopes service response
#[derive(Debug, Clone, Validate, Deserialize)]
pub struct GetUserScopesServiceResponse {
    pub scopes: Vec<ScopeId>,
}

impl From<GetUserScopesRepositoryResponse> for GetUserScopesServiceResponse {
    fn from(response: GetUserScopesRepositoryResponse) -> Self {
        Self {
            scopes: response.scopes.into_iter().map(|scope| scope.id).collect(),
        }
    }
}

/// Add user scope service response
#[derive(Debug, PartialEq, Eq)]
pub struct AddUserScopeServiceResponse {
    pub created: u64,
}

impl From<AddUserScopeRepositoryResponse> for AddUserScopeServiceResponse {
    fn from(response: AddUserScopeRepositoryResponse) -> Self {
        Self {
            created: response.created,
        }
    }
}

/// Remove user scope service response
#[derive(Debug, PartialEq, Eq)]
pub struct RemoveUserScopeServiceResponse {
    pub deleted: u64,
}

impl From<RemoveUserScopeRepositoryResponse> for RemoveUserScopeServiceResponse {
    fn from(response: RemoveUserScopeRepositoryResponse) -> Self {
        Self {
            deleted: response.deleted,
        }
    }
}
