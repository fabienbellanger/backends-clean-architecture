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

pub struct UserService<R: UserRepository> {
    user_repository: R,
}

impl<R: UserRepository> UserService<R> {
    /// Create a new service
    pub fn new(user_repository: R) -> Self {
        Self { user_repository }
    }

    /// Login
    // TODO: Add unit test
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
            .get_user(request)
            .await
            .map(|user| user.into())
    }

    /// Create a user
    // TODO: Add unit test
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::user::User;
    use async_trait::async_trait;
    use uuid::Uuid;

    const DATE: &str = "2023-04-01T12:10:00+00:00";
    const USER_ID: &str = "3288fb86-db99-471d-95bc-1451c7ec6f7b";
    const TOTAL_USERS: i64 = 10;

    struct TestUserRepository {}

    #[async_trait]
    impl UserRepository for TestUserRepository {
        async fn get_users(&self, _paginate_sort: &PaginateSort) -> ApiResult<Vec<User>> {
            let date = DateTime::parse_from_rfc3339(DATE)
                .unwrap()
                .with_timezone(&Utc);

            Ok(vec![User {
                id: Uuid::parse_str(USER_ID).unwrap(),
                lastname: "Doe".to_string(),
                firstname: "John".to_string(),
                email: "john.doe@test.com".to_string(),
                password: "00000000".to_string(),
                created_at: date,
                updated_at: date,
                deleted_at: None,
            }])
        }

        async fn get_user(&self, request: GetUserRequest) -> ApiResult<User> {
            let id = Uuid::parse_str(USER_ID).unwrap();
            if id == request.id {
                let date = DateTime::parse_from_rfc3339(DATE)
                    .unwrap()
                    .with_timezone(&Utc);
                let user = User {
                    id: Uuid::parse_str(USER_ID).unwrap(),
                    lastname: "Doe".to_string(),
                    firstname: "John".to_string(),
                    email: "john.doe@test.com".to_string(),
                    password: "00000000".to_string(),
                    created_at: date,
                    updated_at: date,
                    deleted_at: None,
                };
                Ok(user)
            } else {
                Err(ApiError::InternalError {
                    message: "Not found".to_owned(),
                })
            }
        }

        async fn login(&self, _request: LoginRequest) -> ApiResult<Option<User>> {
            todo!()
        }

        async fn create_user(&self, request: CreateUserRequest) -> ApiResult<User> {
            let date = DateTime::parse_from_rfc3339(DATE)
                .unwrap()
                .with_timezone(&Utc);
            Ok(User {
                id: Uuid::parse_str(USER_ID).unwrap(),
                lastname: request.lastname,
                firstname: request.firstname,
                email: request.email,
                password: request.password,
                created_at: date,
                updated_at: date,
                deleted_at: None,
            })
        }

        async fn delete_user(&self, _request: DeleteUserRequest) -> ApiResult<u64> {
            unimplemented!()
        }

        async fn get_total_users(&self) -> ApiResult<i64> {
            Ok(TOTAL_USERS)
        }
    }

    fn init_service() -> UserService<TestUserRepository> {
        let user_repository = TestUserRepository {};
        UserService::new(user_repository)
    }

    #[tokio::test]
    async fn test_get_user_service() {
        let service = init_service();
        let request = GetUserRequest {
            id: Uuid::parse_str(USER_ID).unwrap(),
        };
        let user = GetUserResponse {
            id: request.id.to_string(),
            lastname: "Doe".to_string(),
            firstname: "John".to_string(),
            email: "john.doe@test.com".to_string(),
            created_at: DATE.to_string(),
            updated_at: DATE.to_string(),
        };

        assert_eq!(service.get_user(request).await, Ok(user));
    }

    #[tokio::test]
    async fn test_get_user_service_no_existing_id() {
        let service = init_service();
        let request = GetUserRequest { id: Uuid::new_v4() };

        assert!(service.get_user(request).await.is_err());
    }

    #[tokio::test]
    async fn test_get_users_service() {
        let service = init_service();
        let users: GetUsersResponse = GetUsersResponse {
            data: vec![GetUserResponse {
                id: Uuid::parse_str(USER_ID).unwrap().to_string(),
                lastname: "Doe".to_string(),
                firstname: "John".to_string(),
                email: "john.doe@test.com".to_string(),
                created_at: DATE.to_string(),
                updated_at: DATE.to_string(),
            }],
            total: TOTAL_USERS,
        };
        let pagination_sort = PaginateSort {
            page: 1,
            limit: 10,
            offset: 0,
            sorts: vec![],
        };
        assert_eq!(service.get_users(&pagination_sort).await, Ok(users));
    }

    #[tokio::test]
    async fn test_create_user_service() {
        let service = init_service();
        let request = CreateUserRequest {
            lastname: "Doe".to_string(),
            firstname: "John".to_string(),
            email: "john.doe@test.com".to_string(),
            password: "00000000".to_string(),
        };
        let user = GetUserResponse {
            id: USER_ID.to_string(),
            lastname: "Doe".to_string(),
            firstname: "John".to_string(),
            email: "john.doe@test.com".to_string(),
            created_at: DATE.to_string(),
            updated_at: DATE.to_string(),
        };

        assert_eq!(service.create_user(request).await, Ok(user));
    }
}
