//! Scopes DTO

use clean_architecture_domain::use_cases::scope::request::CreateScopeUseCaseRequest;
use clean_architecture_domain::use_cases::scope::response::ScopeUseCaseResponse;
use serde::{Deserialize, Serialize};

/// Create a new scope request
#[derive(Debug, Clone, Deserialize)]
pub struct CreateScopeDTORequest {
    pub id: String,
}

impl From<CreateScopeDTORequest> for CreateScopeUseCaseRequest {
    fn from(req: CreateScopeDTORequest) -> Self {
        Self { id: req.id }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeDTOResponse {
    pub id: String,
    pub created_at: String,
}

impl From<ScopeUseCaseResponse> for ScopeDTOResponse {
    fn from(response: ScopeUseCaseResponse) -> Self {
        Self {
            id: response.id.to_string(),
            created_at: response.created_at.to_rfc3339(),
        }
    }
}
