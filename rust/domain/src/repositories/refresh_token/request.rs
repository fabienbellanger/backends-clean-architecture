//! Refresh token repositories requests

use crate::entities::refresh_token::{RefreshToken, RefreshTokenId};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct CreateRefreshTokenRepositoryRequest {
    pub token: RefreshToken,
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetRefreshTokenRepositoryRequest {
    pub token: RefreshTokenId,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RemoveRefreshTokenRepositoryRequest {
    pub token: RefreshTokenId,
}
