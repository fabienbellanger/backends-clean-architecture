//! Users handler module

use crate::extractors::{ExtractRequestId, Path};
use crate::layers::states::SharedState;
use crate::usecases::AppUseCases;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use clean_architecture_domain::ports::requests::user::{
    CreateUserRequest, DeleteUserRequest, GetUserRequest, LoginRequest,
};
use clean_architecture_domain::ports::responses::user::{
    GetUserResponse, GetUsersResponse, LoginResponse,
};
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
use clean_architecture_shared::query_parameter::{PaginateSort, PaginateSortQuery};
use uuid::Uuid;

/// Login route: POST /api/v1/login
#[instrument(skip(uc, state))]
pub async fn login(
    Extension(uc): Extension<AppUseCases>,
    State(state): State<SharedState>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<LoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    let response = uc.user.login(request, &state.jwt).await?;

    Ok(Json(response))
}

/// Users list route: GET /api/v1/users
#[instrument(skip(uc), level = "warn")]
pub async fn get_users(
    Query(pagination): Query<PaginateSortQuery>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<GetUsersResponse>> {
    let paginate_sort = PaginateSort::from(pagination);
    let response = uc.user.get_users(&paginate_sort).await?;

    Ok(Json(response))
}

/// User information route: GET /api/v1/users/:id
#[instrument(skip(uc), level = "warn")]
pub async fn get_user(
    Path(id): Path<Uuid>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<GetUserResponse>> {
    let response = uc.user.get_user(GetUserRequest { id }).await?;

    Ok(Json(response))
}

/// User creation route: POST /api/v1/users
#[instrument(skip(uc), level = "warn")]
pub async fn create_user(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<CreateUserRequest>,
) -> ApiResult<Json<GetUserResponse>> {
    let response = uc.user.create_user(request).await?;

    Ok(Json(response))
}

/// Delete user route: DELETE /api/v1/users/:id
#[instrument(skip(uc), level = "warn")]
pub async fn delete_user(
    Path(id): Path<Uuid>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<StatusCode> {
    let result = uc.user.delete_user(DeleteUserRequest { id }).await?;

    match result {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Err(api_error!(
            ApiErrorCode::NotFound,
            "no user or user already deleted"
        )),
    }
}
