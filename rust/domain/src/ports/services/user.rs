//! User services module

use crate::ports::requests::user::{CreateUserRequest, DeleteUserRequest};
use crate::ports::{
    repositories::user::UserRepository,
    requests::user::{GetUserRequest, LoginRequest},
    responses::user::{GetUserResponse, GetUsersResponse, LoginResponse},
};
use chrono::{DateTime, NaiveDateTime, SecondsFormat, Utc};
use clean_architecture_shared::api_error;
use clean_architecture_shared::auth::Jwt;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
use clean_architecture_shared::query_parameter::PaginateSort;

#[derive(Clone)]
pub struct UserService<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> UserService<R> {
    /// Create a new service
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    /// Login
    #[instrument(skip(self))]
    pub async fn login(&self, request: LoginRequest, jwt: &Jwt) -> ApiResult<LoginResponse> {
        let user = self.user_repository.login(request).await?;

        match user {
            None => Err(api_error!(ApiErrorCode::Unauthorized)),
            Some(user) => {
                // JWT
                let (token, expired_at) = jwt.generate(user.id.to_string())?;
                match NaiveDateTime::from_timestamp_opt(expired_at, 0) {
                    Some(expired_at) => {
                        let expired_at: DateTime<Utc> = DateTime::from_utc(expired_at, Utc);

                        Ok(LoginResponse {
                            id: user.id.to_string(),
                            lastname: user.lastname,
                            firstname: user.firstname,
                            email: user.email,
                            token,
                            expired_at: expired_at.to_rfc3339_opts(SecondsFormat::Secs, true),
                        })
                    }
                    None => Err(api_error!(
                        ApiErrorCode::InternalError,
                        "error during JWT generation",
                        format!(
                            "error during JWT generation: invalid 'expired_at' field in JWT claims ({})",
                            expired_at
                        )
                    )),
                }
            }
        }
    }

    /// Get all users
    #[instrument(skip(self))]
    pub async fn get_users(&self, paginate_sort: &PaginateSort) -> ApiResult<GetUsersResponse> {
        let total = self.user_repository.get_total_users().await?;
        let users = self.user_repository.get_users(paginate_sort).await?;

        Ok((users, total).into())
    }

    /// Get a user
    #[instrument(skip(self))]
    pub async fn get_user(&self, request: GetUserRequest) -> ApiResult<GetUserResponse> {
        self.user_repository
            .get_user_by_id(request)
            .await
            .map(|user| user.into())
    }

    /// Create a user
    #[instrument(skip(self))]
    pub async fn create_user(&self, request: CreateUserRequest) -> ApiResult<GetUserResponse> {
        self.user_repository
            .create_user(request)
            .await
            .map(|user| user.into())
    }

    /// Delete a user
    #[instrument(skip(self))]
    pub async fn delete_user(&self, request: DeleteUserRequest) -> ApiResult<u64> {
        self.user_repository.delete_user(request).await
    }

    /// Get user by email
    #[instrument(skip(self))]
    pub async fn get_user_by_email(&self, email: String) -> ApiResult<GetUserResponse> {
        self.user_repository
            .get_user_by_email(email)
            .await
            .map(|user| user.into())
    }
}
