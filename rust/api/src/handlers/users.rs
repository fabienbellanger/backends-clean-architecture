//! Users handler module

use crate::extractors::ExtractRequestId;
use crate::layers::states::SharedState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use clean_architecture_database::mysql::repositories::user::UserMysqlRepository;
use clean_architecture_domain::ports::requests::user::{
    CreateUserRequest, GetUserRequest, LoginRequest,
};
use clean_architecture_domain::ports::responses::user::{
    GetUserResponse, GetUsersResponse, LoginResponse,
};
use clean_architecture_domain::ports::services::user::UserService;
use clean_architecture_domain::usecases::user::UserUseCase;
use clean_architecture_shared::error::ApiResult;
use sqlx::{MySql, Pool};
use uuid::Uuid;

/// Users handler is used to create user use case
struct UsersHandler<'a> {
    user_use_case: UserUseCase<UserMysqlRepository<'a>>,
}

impl<'a> UsersHandler<'a> {
    /// Create a new `UserHandler`
    pub fn new(pool: &'a Pool<MySql>) -> Self {
        let user_repository = UserMysqlRepository::new(pool);
        let user_service = UserService::new(user_repository);
        let user_use_case = UserUseCase::new(user_service);
        Self { user_use_case }
    }
}

/// Login route: POST /api/v1/login
#[instrument(skip(pool, state))]
pub async fn login(
    Extension(pool): Extension<Pool<MySql>>,
    State(state): State<SharedState>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<LoginRequest>,
) -> ApiResult<Json<LoginResponse>> {
    let handler = UsersHandler::new(&pool);
    let response = handler.user_use_case.login(request, &state.jwt).await?;

    Ok(Json(response))
}

/// Users list route: GET /api/v1/users
#[instrument(skip(pool), level = "warn")]
pub async fn get_users(
    Extension(pool): Extension<Pool<MySql>>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<GetUsersResponse>> {
    let handler = UsersHandler::new(&pool);
    let response = handler.user_use_case.get_users().await?;

    Ok(Json(response))
}

/// User information route: GET /api/v1/users/:id
#[instrument(skip(pool), level = "warn")]
pub async fn get_user(
    Path(id): Path<Uuid>,
    Extension(pool): Extension<Pool<MySql>>,
    ExtractRequestId(request_id): ExtractRequestId,
) -> ApiResult<Json<GetUserResponse>> {
    let handler = UsersHandler::new(&pool);
    let response = handler
        .user_use_case
        .get_user(GetUserRequest { id })
        .await?;

    Ok(Json(response))
}

/// User creation route: POST /api/v1/users
#[instrument(skip(pool), level = "warn")]
pub async fn create_user(
    Extension(pool): Extension<Pool<MySql>>,
    ExtractRequestId(request_id): ExtractRequestId,
    Json(request): Json<CreateUserRequest>,
) -> ApiResult<Json<GetUserResponse>> {
    let handler = UsersHandler::new(&pool);
    let response = handler.user_use_case.create_user(request).await?;

    Ok(Json(response))
}
