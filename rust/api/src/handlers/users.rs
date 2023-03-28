//! Users handler module

use crate::extractors::ExtractRequestId;
use crate::layers::states::SharedState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use clean_architecture_database::mysql::repositories::user::UserMysqlRepository;
use clean_architecture_domain::ports::requests::user::{GetUserRequest, LoginRequest};
use clean_architecture_domain::ports::responses::user::{
    GetUserResponse, GetUsersResponse, LoginResponse,
};
use clean_architecture_domain::ports::services::user::UserService;
use clean_architecture_shared::error::ApiResult;
use sqlx::{MySql, Pool};
use uuid::Uuid;

// Route: GET /api/v1/login
#[instrument(skip(pool, state))]
pub async fn login(
    Extension(pool): Extension<Pool<MySql>>,
    State(state): State<SharedState>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<LoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    let user_repository = UserMysqlRepository::new(&pool);
    let user_service = UserService::new(user_repository);
    let use_case = clean_architecture_domain::usecases::user::UserUseCase::new(user_service);
    let response = use_case.login(request, &state.jwt).await?;

    Ok(Json(response))
}

// Route: GET /api/v1/users
#[instrument(skip(pool), level = "warn")]
pub async fn get_users(
    Extension(pool): Extension<Pool<MySql>>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<GetUsersResponse>> {
    let user_repository = UserMysqlRepository::new(&pool);
    let user_service = UserService::new(user_repository);
    let use_case = clean_architecture_domain::usecases::user::UserUseCase::new(user_service);
    let response = use_case.get_users().await?;

    Ok(Json(response))
}

// Route: GET /api/v1/users/:id
#[instrument(skip(pool), level = "warn")]
pub async fn get_user(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<Pool<MySql>>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<GetUserResponse>> {
    let user_repository = UserMysqlRepository::new(&pool);
    let user_service = UserService::new(user_repository);
    let use_case = clean_architecture_domain::usecases::user::UserUseCase::new(user_service);
    let response = use_case.get_user(GetUserRequest { id }).await?;

    Ok(Json(response))
}
