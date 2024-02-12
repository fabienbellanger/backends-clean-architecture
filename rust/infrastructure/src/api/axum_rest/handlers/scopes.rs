//! Scopes handlers

use crate::api::axum_rest::extractors::{ExtractRequestId, Path};
use crate::api::axum_rest::usecases::AppUseCases;
use axum::http::StatusCode;
use axum::{Extension, Json};
use clean_architecture_domain::ports::requests::scope::{CreateRequest, DeleteRequest};
use clean_architecture_domain::ports::responses::scope::ScopeResponse;
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};

/// Create scope route: POST /api/v1/scopes
#[instrument(skip(uc), name = "create_scope_handler")]
pub async fn create(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<CreateRequest>,
) -> ApiResult<StatusCode> {
    uc.scope.create(request).await?;

    Ok(StatusCode::CREATED)
}

/// Get scopes list route: GET /api/v1/scopes
#[instrument(skip(uc), name = "get_scopes_handler")]
pub async fn get_all(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<Vec<ScopeResponse>>> {
    let scopes = uc.scope.get_scopes().await?;

    Ok(Json(scopes))
}

/// Delete scope route: DELETE /api/v1/scopes/:id
#[instrument(skip(uc), name = "delete_scope_handler")]
pub async fn delete(
    Path(id): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<StatusCode> {
    let result = uc.scope.delete(DeleteRequest { id }).await?;

    match result {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Err(api_error!(ApiErrorCode::NotFound, "no scope or scope already deleted")),
    }
}
