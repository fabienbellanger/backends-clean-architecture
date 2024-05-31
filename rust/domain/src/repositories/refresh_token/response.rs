//! Refresh token repositories responses

use crate::entities::refresh_token::RefreshToken;
use serde::Deserialize;

pub type CreateRefreshTokenRepositoryResponse = ();

#[derive(Debug, Deserialize, Clone)]
pub struct GetRefreshTokenRepositoryResponse {
    pub token: RefreshToken,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RemoveRefreshTokenRepositoryResponse {
    pub deleted: u64,
}
