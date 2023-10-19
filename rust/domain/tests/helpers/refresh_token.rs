use async_trait::async_trait;
use clean_architecture_domain::{
    entities::refresh_token::RefreshToken,
    ports::{
        repositories::refresh_token::RefreshTokenRepository,
        requests::refresh_token::{RefreshTokenId, RefreshTokenRequest},
    },
};
use clean_architecture_shared::error::ApiResult;

pub(crate) struct TestRefreshTokenRepository {}

#[async_trait]
impl RefreshTokenRepository for TestRefreshTokenRepository {
    async fn create(&self, _request: RefreshTokenRequest) -> ApiResult<()> {
        Ok(())
    }

    async fn get(&self, _request: RefreshTokenId) -> ApiResult<RefreshToken> {
        todo!()
    }

    async fn delete(&self, _request: RefreshTokenId) -> ApiResult<()> {
        Ok(())
    }
}
