//! Refresh token model

use chrono::{DateTime, NaiveDateTime, Utc};
use clean_architecture_domain::entities::refresh_token::RefreshToken;
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode};
use sqlx::FromRow;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, FromRow)]
pub struct RefreshTokenModel {
    pub refresh_token: String,
    pub user_id: String,
    pub access_token: String,
    pub expired_at: NaiveDateTime,
}

// TODO: Add tests
impl From<RefreshToken> for RefreshTokenModel {
    fn from(value: RefreshToken) -> Self {
        Self {
            refresh_token: value.refresh_token.to_string(),
            user_id: value.user_id.to_string(),
            access_token: value.access_token,
            expired_at: value.expired_at.naive_utc(),
        }
    }
}

// TODO: Add tests
impl TryFrom<RefreshTokenModel> for RefreshToken {
    type Error = ApiError;

    fn try_from(value: RefreshTokenModel) -> Result<Self, Self::Error> {
        Ok(Self {
            refresh_token: Uuid::from_str(&value.refresh_token)
                .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?,
            user_id: Uuid::from_str(&value.user_id).map_err(|err| api_error!(ApiErrorCode::InternalError, err))?,
            access_token: value.access_token,
            expired_at: DateTime::<Utc>::from_naive_utc_and_offset(value.expired_at, Utc),
        })
    }
}
