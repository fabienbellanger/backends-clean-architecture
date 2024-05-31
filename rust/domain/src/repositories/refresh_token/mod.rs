//! Refresh token repository

pub mod request;
pub mod response;

use crate::repositories::refresh_token::request::{
    CreateRefreshTokenRepositoryRequest, GetRefreshTokenRepositoryRequest, RemoveRefreshTokenRepositoryRequest,
};
use crate::repositories::refresh_token::response::{
    CreateRefreshTokenRepositoryResponse, GetRefreshTokenRepositoryResponse, RemoveRefreshTokenRepositoryResponse,
};
use async_trait::async_trait;
use clean_architecture_shared::error::ApiResult;

#[async_trait]
pub trait RefreshTokenRepository {
    /// Add refresh token
    async fn create(
        &self,
        request: CreateRefreshTokenRepositoryRequest,
    ) -> ApiResult<CreateRefreshTokenRepositoryResponse>;

    /// Get a refresh token
    async fn get(&self, request: GetRefreshTokenRepositoryRequest) -> ApiResult<GetRefreshTokenRepositoryResponse>;

    /// Remove a refresh token
    async fn delete(
        &self,
        request: RemoveRefreshTokenRepositoryRequest,
    ) -> ApiResult<RemoveRefreshTokenRepositoryResponse>;
}
