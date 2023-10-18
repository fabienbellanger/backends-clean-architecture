//! Refresh token requests

use crate::entities::refresh_token::RefreshToken;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct RefreshTokenRequest {
    pub user_id: String,
    pub access_token: String,
    pub refresh_token: String,
    pub expired_at: DateTime<Utc>,
}

impl From<RefreshToken> for RefreshTokenRequest {
    fn from(value: RefreshToken) -> Self {
        Self {
            user_id: value.user_id.to_string(),
            access_token: value.access_token,
            refresh_token: value.refresh_token.to_string(),
            expired_at: value.expired_at,
        }
    }
}
