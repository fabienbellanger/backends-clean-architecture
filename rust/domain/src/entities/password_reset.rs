//! Password Reset entity

use crate::entities::user::UserId;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PasswordReset {
    #[serde(skip_serializing)]
    pub user_id: UserId,
    pub token: String,
    pub expired_at: DateTime<Utc>,
}

impl PasswordReset {
    /// Create a new password recovery
    pub fn new(user_id: UserId, expiration_duration: i64) -> Self {
        let now = Utc::now();
        let expired_at = match Duration::try_hours(expiration_duration) {
            Some(duration) => now.add(duration),
            None => now,
        };

        Self {
            user_id,
            token: Uuid::new_v4().to_string(),
            expired_at,
        }
    }
}
