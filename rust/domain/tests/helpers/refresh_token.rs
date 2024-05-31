use crate::helpers::user::USER_ID;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use clean_architecture_domain::repositories::refresh_token::request::{
    CreateRefreshTokenRepositoryRequest, GetRefreshTokenRepositoryRequest, RemoveRefreshTokenRepositoryRequest,
};
use clean_architecture_domain::repositories::refresh_token::response::{
    CreateRefreshTokenRepositoryResponse, GetRefreshTokenRepositoryResponse, RemoveRefreshTokenRepositoryResponse,
};
use clean_architecture_domain::{
    entities::refresh_token::RefreshToken, repositories::refresh_token::RefreshTokenRepository,
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
    async fn create(
        &self,
        _request: CreateRefreshTokenRepositoryRequest,
    ) -> ApiResult<CreateRefreshTokenRepositoryResponse> {
        Ok(())
    }

    async fn get(&self, request: GetRefreshTokenRepositoryRequest) -> ApiResult<GetRefreshTokenRepositoryResponse> {
        if request.token.to_string() == REFRESH_TOKEN {
            Ok(GetRefreshTokenRepositoryResponse {
                token: RefreshToken {
                    refresh_token: Uuid::try_from(NEW_REFRESH_TOKEN).unwrap(),
                    user_id: Uuid::parse_str(USER_ID).unwrap(),
                    access_token: "access_token".to_owned(),
                    expired_at: Utc::now().add(Duration::try_days(1).unwrap()),
                },
            })
        } else {
            Err(ApiError::Unauthorized)
        }
    }

    async fn delete(
        &self,
        _request: RemoveRefreshTokenRepositoryRequest,
    ) -> ApiResult<RemoveRefreshTokenRepositoryResponse> {
        Ok(RemoveRefreshTokenRepositoryResponse { deleted: 1 })
    }
}
