//! Users handler module

use crate::api::extractors::{ExtractRequestId, Path};
use crate::api::layers::states::SharedState;
use crate::api::usecases::AppUseCases;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use clean_architecture_domain::ports::requests::user::{
    CreateUserRequest, DeleteUserRequest, ForgottenPasswordRequest, GetUserRequest, LoginRequest,
    UpdateUserPasswordRequest,
};
use clean_architecture_domain::ports::responses::password_reset::PasswordResetResponse;
use clean_architecture_domain::ports::responses::user::{
    GetUserResponse, GetUsersResponse, LoginResponse,
};
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
use clean_architecture_shared::query_parameter::{PaginateSort, PaginateSortQuery};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

/// Update user password request body
#[derive(Debug, Validate, Deserialize, Clone)]
pub struct UpdatePasswordRequest {
    #[validate(length(min = 8))]
    pub password: String,
}

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
#[instrument(skip(uc))]
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
#[instrument(skip(uc))]
pub async fn get_user(
    Path(id): Path<Uuid>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<GetUserResponse>> {
    let response = uc.user.get_user(GetUserRequest { id }).await?;

    Ok(Json(response))
}

/// User creation route: POST /api/v1/users
#[instrument(skip(uc))]
pub async fn create_user(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<CreateUserRequest>,
) -> ApiResult<Json<GetUserResponse>> {
    let response = uc.user.create_user(request).await?;

    Ok(Json(response))
}

/// Delete user route: DELETE /api/v1/users/:id
#[instrument(skip(uc))]
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

/// Send forgotten password request: POST /api/v1/forgotten-password/:email
#[instrument(skip(uc, state))]
pub async fn forgotten_password(
    Path(email): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    State(state): State<SharedState>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<PasswordResetResponse>> {
    let result = uc
        .user
        .send_forgotten_password(ForgottenPasswordRequest {
            email,
            expiration_duration: state.config.forgotten_password_expiration_duration,
        })
        .await?;

    Ok(Json(result))
}

/// Update user password: PATCH /api/v1/update-password/:token
#[instrument(skip(uc))]
pub async fn update_password(
    Path(token): Path<Uuid>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(body): Json<UpdatePasswordRequest>,
) -> ApiResult<StatusCode> {
    uc.user
        .update_user_password(UpdateUserPasswordRequest {
            token: token.to_string(),
            password: body.password,
        })
        .await?;

    Ok(StatusCode::NO_CONTENT)
}
