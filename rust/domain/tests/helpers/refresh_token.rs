use async_trait::async_trait;
use clean_architecture_domain::ports::{
    repositories::refresh_token::RefreshTokenRepository, requests::refresh_token::RefreshTokenRequest,
};
use clean_architecture_shared::error::ApiResult;

pub(crate) struct TestRefreshTokenRepository {}

#[async_trait]
impl RefreshTokenRepository for TestRefreshTokenRepository {
    async fn create(&self, _request: RefreshTokenRequest) -> ApiResult<()> {
        Ok(())
    }
}
