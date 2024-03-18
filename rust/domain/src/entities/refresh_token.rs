//! Refresh token entity

use super::user::UserId;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RefreshToken {
    pub refresh_token: Uuid,
    pub user_id: UserId,
    pub access_token: String,
    pub expired_at: DateTime<Utc>,
}

impl RefreshToken {
    /// Create a new refresh token
    pub fn new(user_id: UserId, access_token: &str, expiration_duration: i64) -> Self {
        let now = Utc::now();
        let expired_at = match Duration::try_days(expiration_duration) {
            Some(duration) => now.add(duration),
            None => now,
        };

        Self {
            refresh_token: Uuid::new_v4(),
            user_id,
            access_token: access_token.to_string(),
            expired_at,
        }
    }

    /// Check if the token is valid (now > expired datetime)
    pub fn is_valid(&self) -> bool {
        let now = Utc::now();

        now <= self.expired_at
    }
}
