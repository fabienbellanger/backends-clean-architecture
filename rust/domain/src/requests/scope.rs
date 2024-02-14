//! Scope requests

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct CreateRequest {
    #[validate(length(min = 4))]
    pub id: String,
}

#[derive(Debug, Clone, Validate, Deserialize)]
pub struct DeleteRequest {
    #[validate(length(min = 4))]
    pub id: String,
}
