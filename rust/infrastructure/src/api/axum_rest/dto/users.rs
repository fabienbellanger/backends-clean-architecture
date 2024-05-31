//! Users DTO

use serde::Serialize;

/// Create a new scope request
#[derive(Debug, Clone, Serialize)]
pub struct GetRefreshTokenDTOResponse {
    pub token: String,
}
