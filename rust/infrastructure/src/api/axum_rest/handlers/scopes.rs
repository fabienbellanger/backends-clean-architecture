//! Scopes handlers

use crate::api::axum_rest::dto::scopes::{CreateScopeDTORequest, ScopeDTOResponse};
use crate::api::axum_rest::extractors::{ExtractRequestId, Path};
use crate::api::axum_rest::use_cases::AppUseCases;
use axum::http::StatusCode;
use axum::{Extension, Json};
use clean_architecture_domain::use_cases::scope::request::DeleteScopeUseCaseRequest;
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};

/// Create scope route: POST /api/v1/scopes
#[instrument(skip(uc), name = "create_scope_handler")]
pub async fn create(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<CreateScopeDTORequest>,
) -> ApiResult<StatusCode> {
    uc.scope.create(request.into()).await?;

    Ok(StatusCode::CREATED)
}

/// Get scopes list route: GET /api/v1/scopes
#[instrument(skip(uc), name = "get_scopes_handler")]
pub async fn get_all(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<Vec<ScopeDTOResponse>>> {
    let scopes = uc
        .scope
        .get_scopes()
        .await?
        .scopes
        .into_iter()
        .map(|scope| scope.into())
        .collect();

    Ok(Json(scopes))
}

/// Delete scope route: DELETE /api/v1/scopes/:id
#[instrument(skip(uc), name = "delete_scope_handler")]
pub async fn delete(
    Path(id): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<StatusCode> {
    let result = uc.scope.delete(DeleteScopeUseCaseRequest { id }).await?;

    match result.deleted {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Err(api_error!(ApiErrorCode::NotFound, "no scope or scope already deleted")),
    }
}
