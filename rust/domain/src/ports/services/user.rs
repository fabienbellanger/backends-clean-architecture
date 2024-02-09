//! User services module

use crate::entities::password_reset::PasswordReset;
use crate::entities::refresh_token::RefreshToken;
use crate::ports::repositories::password_reset::PasswordResetRepository;
use crate::ports::repositories::refresh_token::RefreshTokenRepository;
use crate::ports::requests::password_reset::{DeleteRequest, GetByTokenRequest};
use crate::ports::requests::refresh_token::{RefreshTokenHttpRequest, RefreshTokenId};
use crate::ports::requests::user::{
    CreateUserRequest, DeleteUserRequest, ForgottenPasswordRequest, UpdateUserPasswordRepositoryRequest,
    UpdateUserPasswordRequest,
};
use crate::ports::responses::refresh_token::RefreshTokenResponse;
use crate::ports::{
    repositories::user::UserRepository,
    requests::user::{GetUserRequest, LoginRequest},
    responses::user::{GetUserResponse, GetUsersResponse, LoginResponse},
};
use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};
use clean_architecture_shared::api_error;
use clean_architecture_shared::auth::{AuthScope, Jwt};
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
use clean_architecture_shared::query_parameter::PaginateSort;

#[derive(Clone)]
pub struct UserService<U: UserRepository, P: PasswordResetRepository, T: RefreshTokenRepository> {
    user_repository: U,
    password_reset_repository: P,
    refresh_token_repository: T,
}

impl<U: UserRepository, P: PasswordResetRepository, T: RefreshTokenRepository> UserService<U, P, T> {
    /// Create a new service
    pub fn new(user_repository: U, password_reset_repository: P, refresh_token_repository: T) -> Self {
        Self {
            user_repository,
            password_reset_repository,
            refresh_token_repository,
        }
    }

    /// Login
    #[instrument(skip(self), name = "user_service_login")]
    pub async fn login(&self, request: LoginRequest, jwt: &Jwt) -> ApiResult<LoginResponse> {
        let user = self.user_repository.login(request).await?;

        match user {
            None => Err(api_error!(ApiErrorCode::Unauthorized)),
            Some(user) => {
                // Scopes
                let scopes = self
                    .user_repository
                    .get_scopes(user.id)
                    .await?
                    .into_iter()
                    .map(AuthScope::from)
                    .collect::<Vec<AuthScope>>();

                // JWT
                let (access_token, access_expired_at) = jwt.generate(user.id.to_string(), scopes)?;
                match NaiveDateTime::from_timestamp_opt(access_expired_at, 0) {
                    Some(access_expired_at) => {
                        let access_expired_at: DateTime<Utc> =
                            DateTime::from_naive_utc_and_offset(access_expired_at, Utc);

                        // Generate refresh token
                        let refresh_token = RefreshToken::new(user.id, &access_token, jwt.refresh_lifetime);

                        // Save refresh token
                        self.refresh_token_repository
                            .create(refresh_token.clone().into())
                            .await?;

                        Ok(LoginResponse {
                            id: user.id.to_string(),
                            lastname: user.lastname,
                            firstname: user.firstname,
                            email: user.email.value(),
                            access_token,
                            access_token_expired_at: access_expired_at.to_rfc3339_opts(SecondsFormat::Secs, true),
                            refresh_token: refresh_token.refresh_token.to_string(),
                            refresh_token_expired_at: refresh_token
                                .expired_at
                                .to_rfc3339_opts(SecondsFormat::Secs, true),
                        })
                    }
                    None => Err(api_error!(
                        ApiErrorCode::InternalError,
                        "error during JWT generation",
                        format!(
                            "error during JWT generation: invalid 'expired_at' field in JWT claims ({access_expired_at})"
                        )
                    )),
                }
            }
        }
    }

    /// Get all users
    #[instrument(skip(self), name = "user_service_get_users")]
    pub async fn refresh_token(&self, request: RefreshTokenHttpRequest, jwt: &Jwt) -> ApiResult<RefreshTokenResponse> {
        let refresh_token_id = RefreshTokenId {
            refresh_token: request.refresh_token,
        };

        let refresh_token = self.refresh_token_repository.get(refresh_token_id.clone()).await?;

        // Remove old refresh token
        self.refresh_token_repository.delete(refresh_token_id).await?;

        match refresh_token.is_valid() {
            false => Err(api_error!(ApiErrorCode::Unauthorized)),
            true => {
                // Scopes
                let scopes = self
                    .user_repository
                    .get_scopes(refresh_token.user_id)
                    .await?
                    .into_iter()
                    .map(AuthScope::from)
                    .collect::<Vec<AuthScope>>();

                // Generate new access and refresh token
                let (access_token, access_expired_at) = jwt.generate(refresh_token.user_id.to_string(), scopes)?;
                match NaiveDateTime::from_timestamp_opt(access_expired_at, 0) {
                    Some(access_expired_at) => {
                        let access_expired_at: DateTime<Utc> =
                            DateTime::from_naive_utc_and_offset(access_expired_at, Utc);

                        // Generate refresh token
                        let refresh_token =
                            RefreshToken::new(refresh_token.user_id, &access_token, jwt.refresh_lifetime);

                        // Save refresh token
                        self.refresh_token_repository
                            .create(refresh_token.clone().into())
                            .await?;

                        Ok(RefreshTokenResponse {
                            access_token,
                            access_token_expired_at: access_expired_at.to_rfc3339_opts(SecondsFormat::Secs, true),
                            refresh_token: refresh_token.refresh_token.to_string(),
                            refresh_token_expired_at: refresh_token
                                .expired_at
                                .to_rfc3339_opts(SecondsFormat::Secs, true),
                        })
                    }
                    None => Err(api_error!(
                        ApiErrorCode::InternalError,
                        "error during JWT generation",
                        format!(
                            "error during JWT generation: invalid 'expired_at' field in JWT claims ({})",
                            access_expired_at
                        )
                    )),
                }
            }
        }
    }

    /// Get all users
    #[instrument(skip(self), name = "user_service_get_users")]
    pub async fn get_users(&self, paginate_sort: &PaginateSort) -> ApiResult<GetUsersResponse> {
        let total = self.user_repository.get_total_users().await?;
        let users = self.user_repository.get_users(paginate_sort).await?;

        Ok((users, total).into())
    }

    /// Get a user
    #[instrument(skip(self), name = "user_service_get_user")]
    pub async fn get_user(&self, request: GetUserRequest) -> ApiResult<GetUserResponse> {
        self.user_repository
            .get_user_by_id(request)
            .await
            .map(|user| user.into())
    }

    /// Create a user
    #[instrument(skip(self), name = "user_service_create_user")]
    pub async fn create_user(&self, request: CreateUserRequest) -> ApiResult<GetUserResponse> {
        self.user_repository.create_user(request).await.map(|user| user.into())
    }

    /// Delete a user
    #[instrument(skip(self), name = "user_service_delete_user")]
    pub async fn delete_user(&self, request: DeleteUserRequest) -> ApiResult<u64> {
        // TODO: Check if user to delete is different of user who make the deletion
        self.user_repository.delete_user(request).await
    }

    /// Forgotten password request
    #[instrument(skip(self), name = "user_service_forgotten_password")]
    pub async fn forgotten_password(&self, request: ForgottenPasswordRequest) -> ApiResult<PasswordReset> {
        let user = self.user_repository.get_user_by_email(request.email.clone()).await?;

        // Password reset
        let password_reset = PasswordReset::new(user.id, request.expiration_duration);

        self.password_reset_repository
            .create_or_update(password_reset.clone().into())
            .await?;

        Ok(password_reset)
    }

    // Update user password
    #[instrument(skip(self), name = "user_service_update_user_password")]
    pub async fn update_user_password(&self, request: UpdateUserPasswordRequest) -> ApiResult<()> {
        let result = self
            .password_reset_repository
            .get_by_token(GetByTokenRequest {
                token: request.token.clone(),
            })
            .await?;

        match result {
            Some(user_id) => {
                // Update user password
                self.user_repository
                    .update_password(UpdateUserPasswordRepositoryRequest {
                        id: user_id.clone(),
                        password: request.password,
                    })
                    .await?;

                // Delete password reset entry
                self.password_reset_repository.delete(DeleteRequest { user_id }).await?;

                Ok(())
            }
            None => Err(api_error!(ApiErrorCode::NotFound, "no user found")),
        }
    }
}
