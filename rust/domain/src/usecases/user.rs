//! User use cases

use crate::ports::repositories::password_reset::PasswordResetRepository;
use crate::ports::requests::user::{
    CreateUserRequest, DeleteUserRequest, ForgottenPasswordRequest, UpdateUserPasswordRequest,
};
use crate::ports::responses::password_reset::PasswordResetResponse;
use crate::ports::services::email::EmailService;
use crate::ports::{
    repositories::user::UserRepository,
    requests::user::{GetUserRequest, LoginRequest},
    responses::user::{GetUserResponse, GetUsersResponse, LoginResponse},
    services::user::UserService,
};
use chrono::SecondsFormat;
use clean_architecture_shared::auth::Jwt;
use clean_architecture_shared::error::ApiResult;
use clean_architecture_shared::query_parameter::PaginateSort;
use clean_architecture_shared::validation::validate_request_data;
use std::sync::Arc;

#[derive(Clone)]
/// Create new user use case
pub struct UserUseCase<U, P, E>
where
    U: UserRepository,
    P: PasswordResetRepository,
    E: EmailService,
{
    user_service: Arc<UserService<U, P>>,
    email_service: Arc<E>,
}

impl<U, P, E> UserUseCase<U, P, E>
where
    U: UserRepository,
    P: PasswordResetRepository,
    E: EmailService,
{
    /// Create a new use case
    pub fn new(user_service: Arc<UserService<U, P>>, email_service: Arc<E>) -> Self {
        Self {
            user_service,
            email_service,
        }
    }

    /// Login
    #[instrument(skip(self))]
    pub async fn login(&self, request: LoginRequest, jwt: &Jwt) -> ApiResult<LoginResponse> {
        validate_request_data(&request)?;

        self.user_service.login(request, jwt).await
    }

    /// Get all users
    #[instrument(skip(self))]
    pub async fn get_users(&self, paginate_sort: &PaginateSort) -> ApiResult<GetUsersResponse> {
        self.user_service.get_users(paginate_sort).await
    }

    /// Get a user
    #[instrument(skip(self))]
    pub async fn get_user(&self, request: GetUserRequest) -> ApiResult<GetUserResponse> {
        validate_request_data(&request)?;

        self.user_service.get_user(request).await
    }

    /// Create user
    #[instrument(skip(self))]
    pub async fn create_user(&self, request: CreateUserRequest) -> ApiResult<GetUserResponse> {
        validate_request_data(&request)?;

        self.user_service.create_user(request).await
    }

    /// Delete a user
    #[instrument(skip(self))]
    pub async fn delete_user(&self, request: DeleteUserRequest) -> ApiResult<u64> {
        validate_request_data(&request)?;

        self.user_service.delete_user(request).await
    }

    /// Send forgotten password request
    #[instrument(skip(self))]
    pub async fn send_forgotten_password(&self, request: ForgottenPasswordRequest) -> ApiResult<PasswordResetResponse> {
        validate_request_data(&request)?;

        let password_reset = self.user_service.forgotten_password(request.clone()).await?;

        // Send email
        self.email_service.forgotten_password(request, &password_reset.token)?;

        Ok(PasswordResetResponse {
            token: password_reset.token,
            expired_at: password_reset.expired_at.to_rfc3339_opts(SecondsFormat::Secs, true),
        })
    }

    /// Update user password
    #[instrument(skip(self))]
    pub async fn update_user_password(&self, request: UpdateUserPasswordRequest) -> ApiResult<()> {
        validate_request_data(&request)?;

        self.user_service.update_user_password(request).await
    }
}
