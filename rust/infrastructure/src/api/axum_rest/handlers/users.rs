//! Users handlers

use crate::api::axum_rest::extractors::{ExtractRequestId, Path};
use crate::api::axum_rest::layers::states::SharedState;
use crate::api::axum_rest::usecases::AppUseCases;
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use clean_architecture_domain::entities::scope::ScopeId;
use clean_architecture_domain::requests::refresh_token::RefreshTokenHttpRequest;
use clean_architecture_domain::requests::user::{
    CreateUserRequest, DeleteUserRequest, ForgottenPasswordRequest, LoginRequest, UpdateUserPasswordRequest,
    UserIdRequest, UserScopeRequest,
};
use clean_architecture_domain::responses::password_reset::PasswordResetResponse;
use clean_architecture_domain::responses::refresh_token::RefreshTokenResponse;
use clean_architecture_domain::responses::user::{GetUserResponse, GetUsersResponse, LoginResponse};
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

/// Scope ID request
#[derive(Debug, Validate, Deserialize, Clone)]
pub struct ScopeIdRequest {
    #[validate(length(min = 4))]
    pub id: String,
}

/// Login route: POST /api/v1/login
#[instrument(skip(uc, state), name = "login_handler")]
pub async fn login(
    Extension(uc): Extension<AppUseCases>,
    State(state): State<SharedState>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<LoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    let response = uc.user.login(request, &state.jwt).await?;

    Ok(Json(response))
}

/// Refresh token route: POST /api/v1/refresh-token/:token
#[instrument(skip(uc, state), name = "refresh_token_handler")]
pub async fn refresh_token(
    Path(refresh_token): Path<String>,
    Extension(uc): Extension<AppUseCases>,
    State(state): State<SharedState>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<RefreshTokenResponse>> {
    let response = uc
        .user
        .refresh_token(RefreshTokenHttpRequest { refresh_token }, &state.jwt)
        .await?;

    Ok(Json(response))
}

/// Users list route: GET /api/v1/users
#[instrument(skip(uc), name = "get_users_handler")]
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
#[instrument(skip(uc), name = "get_user_handler")]
pub async fn get_user(
    Path(id): Path<Uuid>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<GetUserResponse>> {
    let response = uc.user.get_user(UserIdRequest { id }).await?;

    Ok(Json(response))
}

/// User creation route: POST /api/v1/users
#[instrument(skip(uc), name = "create_user_handler")]
pub async fn create_user(
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<CreateUserRequest>,
) -> ApiResult<Json<GetUserResponse>> {
    let response = uc.user.create_user(request).await?;

    Ok(Json(response))
}

/// Delete user route: DELETE /api/v1/users/:id
#[instrument(skip(uc), name = "delete_user_handler")]
pub async fn delete_user(
    Path(id): Path<Uuid>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<StatusCode> {
    let result = uc.user.delete_user(DeleteUserRequest { id }).await?;

    match result {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Err(api_error!(ApiErrorCode::NotFound, "no user or user already deleted")),
    }
}

/// Send forgotten password request: POST /api/v1/forgotten-password/:email
#[instrument(skip(uc, state), name = "forgotten_password_handler")]
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
#[instrument(skip(uc), name = "update_password_handler")]
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

/// Get scopes: GET /api/v1/users/:id/scopes
#[instrument(skip(uc), name = "get_user_scopes_handler")]
pub async fn get_scopes(
    Path(id): Path<Uuid>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<Vec<ScopeId>>> {
    let scopes = uc.user.get_scopes(UserIdRequest { id }).await?;

    Ok(Json(scopes))
}

/// Add scope: POST /api/v1/users/:id/scopes
#[instrument(skip(uc), name = "user_add_scope_handler")]
pub async fn add_scope(
    Path(id): Path<Uuid>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(body): Json<ScopeIdRequest>,
) -> ApiResult<StatusCode> {
    let result = uc
        .user
        .add_scope(UserScopeRequest {
            user_id: id,
            scope_id: body.id,
        })
        .await?;

    match result {
        1 => Ok(StatusCode::CREATED),
        _ => Err(api_error!(
            ApiErrorCode::NotFound,
            "user or scope does not exist or already added to user"
        )),
    }
}

/// Remove scope: DELETE /api/v1/users/:id/scopes/:scope_id
#[instrument(skip(uc), name = "user_remove_scope_handler")]
pub async fn remove_scope(
    Path(path): Path<(Uuid, ScopeId)>,
    Extension(uc): Extension<AppUseCases>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<StatusCode> {
    let result = uc
        .user
        .remove_scope(UserScopeRequest {
            user_id: path.0,
            scope_id: path.1,
        })
        .await?;

    match result {
        1 => Ok(StatusCode::NO_CONTENT),
        _ => Err(api_error!(
            ApiErrorCode::NotFound,
            "user or scope does not exist or already removed from user"
        )),
    }
}
