//! Refresh token repository

use crate::{
    entities::refresh_token::RefreshToken,
    ports::requests::refresh_token::{RefreshTokenId, RefreshTokenRequest},
};
use async_trait::async_trait;
use clean_architecture_shared::error::ApiResult;

#[async_trait]
pub trait RefreshTokenRepository {
    /// Add refresh token
    async fn create(&self, request: RefreshTokenRequest) -> ApiResult<()>;

    /// Get a refresh token
    async fn get(&self, request: RefreshTokenId) -> ApiResult<RefreshToken>;

    /// Remove a refresh token
    async fn delete(&self, request: RefreshTokenId) -> ApiResult<()>;
}
