//! Password Reset requests modules

use crate::entities::password_reset::PasswordReset;
use chrono::{DateTime, Utc};
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
pub struct PasswordResetRequest {
    pub user_id: Uuid,
    pub token: String,
    pub expired_at: DateTime<Utc>,
}

impl TryFrom<PasswordReset> for PasswordResetRequest {
    type Error = ApiError;

    fn try_from(value: PasswordReset) -> Result<Self, Self::Error> {
        Ok(Self {
            user_id: Uuid::parse_str(&value.user_id).map_err(|err| api_error!(ApiErrorCode::BadRequest, err))?,
            token: value.token,
            expired_at: value.expired_at,
        })
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct GetByTokenRequest {
    pub token: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct DeleteRequest {
    pub user_id: String,
}
