//! Password Reset entity

use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct PasswordReset {
    #[serde(skip_serializing)]
    pub user_id: String,
    pub token: String,
    pub expired_at: DateTime<Utc>,
}

impl PasswordReset {
    /// Create a new password recovery
    // TODO: Add test
    pub fn new(user_id: String, expiration_duration: i64) -> Self {
        let now = Utc::now();

        Self {
            user_id,
            token: Uuid::new_v4().to_string(),
            expired_at: now.add(Duration::hours(expiration_duration)),
        }
    }
}
