//! Refresh token repository

use crate::ports::requests::refresh_token::RefreshTokenRequest;
use async_trait::async_trait;
use clean_architecture_shared::error::ApiResult;

#[async_trait]
pub trait RefreshTokenRepository {
    /// Add refresh token
    async fn create(&self, request: RefreshTokenRequest) -> ApiResult<()>;
}
