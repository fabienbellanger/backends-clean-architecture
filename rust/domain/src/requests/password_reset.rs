//! Password Reset requests

use crate::entities::password_reset::PasswordReset;
use crate::entities::user::UserId;
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct PasswordResetRequest {
    pub user_id: UserId,
    pub token: String,
    pub expired_at: DateTime<Utc>,
}

impl From<PasswordReset> for PasswordResetRequest {
    fn from(value: PasswordReset) -> Self {
        Self {
            user_id: value.user_id,
            token: value.token,
            expired_at: value.expired_at,
        }
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
