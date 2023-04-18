//! Password resset responses modules

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordResetResponse {
    pub token: String,
    pub expired_at: String,
}
