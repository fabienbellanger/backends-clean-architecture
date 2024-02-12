//! Handlers module

pub(crate) mod scopes;
pub(crate) mod users;
pub(crate) mod web;

use axum::BoxError;
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
use tower::timeout::error::Elapsed;

/// Timeout error
pub async fn timeout_error(err: BoxError) -> ApiResult<()> {
    if err.is::<Elapsed>() {
        Err(api_error!(ApiErrorCode::Timeout))
    } else {
        Err(api_error!(ApiErrorCode::InternalError, err.to_string()))
    }
}
