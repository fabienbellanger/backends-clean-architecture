//! User use cases

use crate::repositories::password_reset::PasswordResetRepository;
use crate::repositories::refresh_token::RefreshTokenRepository;
use crate::requests::refresh_token::RefreshTokenHttpRequest;
use crate::requests::user::{
    CreateUserRequest, DeleteUserRequest, ForgottenPasswordRequest, UpdateUserPasswordRequest, UserScopeRequest,
};
use crate::responses::password_reset::PasswordResetResponse;
use crate::responses::refresh_token::RefreshTokenResponse;
use crate::services::email::EmailService;
use crate::{
    repositories::user::UserRepository,
    requests::user::{LoginRequest, UserIdRequest},
    responses::user::{GetUserResponse, GetUsersResponse, LoginResponse},
    services::user::UserService,
};
use chrono::SecondsFormat;
use clean_architecture_shared::auth::Jwt;
use clean_architecture_shared::error::ApiResult;
use clean_architecture_shared::query_parameter::PaginateSort;
use clean_architecture_shared::validation::validate_request_data;

/// Create new user use case
#[derive(Clone)]
pub struct UserUseCase<U, P, T, E>
where
    U: UserRepository,
    P: PasswordResetRepository,
    T: RefreshTokenRepository,
    E: EmailService,
{
    user_service: UserService<U, P, T>,
    email_service: E,
}

impl<U, P, E, T> UserUseCase<U, P, T, E>
where
    U: UserRepository,
    P: PasswordResetRepository,
    T: RefreshTokenRepository,
    E: EmailService,
{
    /// Create a new use case
    pub fn new(user_service: UserService<U, P, T>, email_service: E) -> Self {
        Self {
            user_service,
            email_service,
        }
    }

    /// Login
    #[instrument(skip(self), name = "user_use_case_login")]
    pub async fn login(&self, request: LoginRequest, jwt: &Jwt) -> ApiResult<LoginResponse> {
        validate_request_data(&request)?;

        self.user_service.login(request, jwt).await
    }

    /// Refresh token
    #[instrument(skip(self), name = "user_use_case_refresh_token")]
    pub async fn refresh_token(&self, request: RefreshTokenHttpRequest, jwt: &Jwt) -> ApiResult<RefreshTokenResponse> {
        self.user_service.refresh_token(request, jwt).await
    }

    /// Get all users
    #[instrument(skip(self), name = "user_use_case_get_users")]
    pub async fn get_users(&self, paginate_sort: &PaginateSort) -> ApiResult<GetUsersResponse> {
        self.user_service.get_users(paginate_sort).await
    }

    /// Get a user
    #[instrument(skip(self), name = "user_use_case_get_user")]
    pub async fn get_user(&self, request: UserIdRequest) -> ApiResult<GetUserResponse> {
        validate_request_data(&request)?;

        self.user_service.get_user(request).await
    }

    /// Create user
    #[instrument(skip(self), name = "user_use_case_create_user")]
    pub async fn create_user(&self, request: CreateUserRequest) -> ApiResult<GetUserResponse> {
        validate_request_data(&request)?;

        self.user_service.create_user(request).await
    }

    /// Delete a user
    #[instrument(skip(self), name = "user_use_case_delete_user")]
    pub async fn delete_user(&self, request: DeleteUserRequest) -> ApiResult<u64> {
        validate_request_data(&request)?;

        self.user_service.delete_user(request).await
    }

    /// Send forgotten password request
    #[instrument(skip(self), name = "user_use_case_send_forgotten_password")]
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
    #[instrument(skip(self), name = "user_use_case_update_user_password")]
    pub async fn update_user_password(&self, request: UpdateUserPasswordRequest) -> ApiResult<()> {
        validate_request_data(&request)?;

        self.user_service.update_user_password(request).await
    }

    /// Get active scopes of a user
    #[instrument(skip(self), name = "user_use_case_get_scopes")]
    pub async fn get_scopes(&self, request: UserIdRequest) -> ApiResult<Vec<String>> {
        validate_request_data(&request)?;

        self.user_service.get_scopes(request).await
    }

    /// Add a scope to a user
    #[instrument(skip(self), name = "user_use_case_add_scope")]
    pub async fn add_scope(&self, request: UserScopeRequest) -> ApiResult<u64> {
        validate_request_data(&request)?;

        self.user_service.add_scope(request).await
    }

    /// Remove a scope to a user
    #[instrument(skip(self), name = "user_use_case_remove_scope")]
    pub async fn remove_scope(&self, request: UserScopeRequest) -> ApiResult<u64> {
        validate_request_data(&request)?;

        self.user_service.remove_scope(request).await
    }
}
