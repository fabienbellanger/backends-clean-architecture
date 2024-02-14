//! HTTP request validation

use super::error::{ApiError, ApiErrorCode, ApiResult};
use crate::api_error;
use serde_json::json;
use validator::Validate;

/// Validate the HTTP request parameters
pub fn validate_request_data<T: Validate>(data: &T) -> ApiResult<()> {
    match data.validate() {
        Ok(_) => Ok(()),
        Err(errors) => Err(api_error!(ApiErrorCode::BadRequest, json!(errors).to_string())),
    }
}
