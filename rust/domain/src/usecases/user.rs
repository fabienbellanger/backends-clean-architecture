//! User use cases

use crate::entities::password_reset::PasswordReset;
use crate::ports::requests::user::{
    CreateUserRequest, DeleteUserRequest, ForgottenPasswordRequest,
};
use crate::ports::services::email::EmailService;
use crate::ports::{
    repositories::user::UserRepository,
    requests::user::{GetUserRequest, LoginRequest},
    responses::user::{GetUserResponse, GetUsersResponse, LoginResponse},
    services::user::UserService,
};
use clean_architecture_shared::auth::Jwt;
use clean_architecture_shared::error::ApiResult;
use clean_architecture_shared::query_parameter::PaginateSort;
use clean_architecture_shared::validation::validate_request_data;

#[derive(Clone)]
/// Create new user use case
pub struct UserUseCase<R: UserRepository, E: EmailService> {
    user_service: UserService<R>,
    email_service: E,
}

impl<R, E> UserUseCase<R, E>
where
    R: UserRepository,
    E: EmailService,
{
    /// Create a new use case
    pub fn new(user_service: UserService<R>, email_service: E) -> Self {
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
    pub async fn send_forgotten_password(
        &self,
        request: ForgottenPasswordRequest,
    ) -> ApiResult<()> {
        // Get user from email
        let user = self
            .user_service
            .forgotten_password(request.email.clone())
            .await?;

        // Password reset
        let password_reset = PasswordReset::new(user.id, request.expiration_duration);

        // TODO: Insert in `password_resets` table

        // Use EmailService
        self.email_service
            .forgotten_password(request, &password_reset.token)?;
        Ok(())
    }
}
