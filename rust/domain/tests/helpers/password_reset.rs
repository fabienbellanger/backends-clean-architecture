use async_trait::async_trait;
use clean_architecture_domain::ports::{
    repositories::password_reset::PasswordResetRepository,
    requests::password_reset::{DeleteRequest, GetByTokenRequest, PasswordResetRequest},
};
use clean_architecture_shared::error::ApiResult;

pub(crate) struct TestPasswordResetRepository {}

#[async_trait]
impl PasswordResetRepository for TestPasswordResetRepository {
    /// Add forgotten password request
    async fn create_or_update(&self, _request: PasswordResetRequest) -> ApiResult<()> {
        Ok(())
    }

    /// Get by token and return a tuple of user ID and user password
    async fn get_by_token(&self, _request: GetByTokenRequest) -> ApiResult<Option<String>> {
        Ok(None)
    }

    /// Delete
    async fn delete(&self, _request: DeleteRequest) -> ApiResult<u64> {
        Ok(0)
    }
}
