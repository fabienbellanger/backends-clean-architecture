use super::user::USER_ID;
use async_trait::async_trait;
use clean_architecture_domain::ports::{
    repositories::password_reset::PasswordResetRepository,
    requests::password_reset::{DeleteRequest, GetByTokenRequest, PasswordResetRequest},
};
use clean_architecture_shared::error::ApiResult;

pub(crate) const FORGOTTEN_PASSWORD_TOKEN: &str = "3288fb86-db99-471d-95bc-1451c7ec6f7c";

pub(crate) struct TestPasswordResetRepository {}

#[async_trait]
impl PasswordResetRepository for TestPasswordResetRepository {
    /// Add forgotten password request
    async fn create_or_update(&self, _request: PasswordResetRequest) -> ApiResult<()> {
        Ok(())
    }

    /// Get by token and return a tuple of user ID and user password
    async fn get_by_token(&self, request: GetByTokenRequest) -> ApiResult<Option<String>> {
        if &request.token == FORGOTTEN_PASSWORD_TOKEN {
            return Ok(Some(USER_ID.to_owned()));
        }
        Ok(None)
    }

    /// Delete
    async fn delete(&self, _request: DeleteRequest) -> ApiResult<u64> {
        Ok(0)
    }
}
