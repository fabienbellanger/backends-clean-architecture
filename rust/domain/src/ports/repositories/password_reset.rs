//! Password Reset repository module

use crate::ports::requests::password_reset::{
    DeleteRequest, GetByTokenRequest, PasswordResetRequest,
};
use async_trait::async_trait;
use clean_architecture_shared::error::ApiResult;

#[async_trait]
pub trait PasswordResetRepository {
    /// Add forgotten password request
    async fn create_or_update(&self, request: PasswordResetRequest) -> ApiResult<()>;

    /// Get by token and return a tuple of user ID and user password
    async fn get_by_token(&self, request: GetByTokenRequest) -> ApiResult<Option<String>>;

    /// Delete forgotten password entry
    async fn delete(&self, request: DeleteRequest) -> ApiResult<u64>;
}
