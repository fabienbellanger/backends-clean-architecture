//! Web handler module

use crate::api::TEMPLATES;
use axum::response::Html;
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
use tera::Context;

/// Health check route: GET "/health-check"
pub async fn health_check<'a>() -> &'a str {
    "OK"
}

/// API documentation route: GET "/doc/api-v1"
pub async fn doc_api_v1() -> ApiResult<Html<String>> {
    let templates = TEMPLATES.as_ref().map_err(|err| {
        api_error!(
            ApiErrorCode::InternalError,
            "error during template render",
            err
        )
    })?;
    Ok(Html(
        templates
            .render("doc/api_v1.html", &Context::new())
            .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?,
    ))
}
