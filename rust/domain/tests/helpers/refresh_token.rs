use crate::helpers::user::USER_ID;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use clean_architecture_domain::{
    entities::refresh_token::RefreshToken,
    {
        repositories::refresh_token::RefreshTokenRepository,
        requests::refresh_token::{RefreshTokenId, RefreshTokenRequest},
    },
};
use clean_architecture_shared::error::{ApiError, ApiResult};
use std::ops::Add;
use uuid::Uuid;

pub(crate) const REFRESH_TOKEN: &str = "3288fb86-db99-471d-95bc-1451c7ec6f7c";
pub(crate) const NEW_REFRESH_TOKEN: &str = "3288fb86-db99-471d-95bc-1451c7ec6f71";
pub(crate) const INVALID_REFRESH_TOKEN: &str = "3288fb86-db99-471d-95bc-1451c7ec6f7e";

pub(crate) struct TestRefreshTokenRepository {}

#[async_trait]
impl RefreshTokenRepository for TestRefreshTokenRepository {
    async fn create(&self, _request: RefreshTokenRequest) -> ApiResult<()> {
        Ok(())
    }

    async fn get(&self, request: RefreshTokenId) -> ApiResult<RefreshToken> {
        if request.refresh_token == REFRESH_TOKEN {
            Ok(RefreshToken {
                refresh_token: Uuid::try_from(NEW_REFRESH_TOKEN).unwrap(),
                user_id: Uuid::parse_str(USER_ID).unwrap(),
                access_token: "access_token".to_owned(),
                expired_at: Utc::now().add(Duration::days(1)),
            })
        } else {
            Err(ApiError::Unauthorized)
        }
    }

    async fn delete(&self, _request: RefreshTokenId) -> ApiResult<()> {
        Ok(())
    }
}
