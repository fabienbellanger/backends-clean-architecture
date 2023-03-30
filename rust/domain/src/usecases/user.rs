//! User use cases
#![allow(dead_code)]

use crate::ports::requests::user::{CreateUserRequest, DeleteUserRequest};
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

/// Create new user use case
pub struct UserUseCase<R: UserRepository> {
    user_service: UserService<R>,
}

impl<R> UserUseCase<R>
where
    R: UserRepository,
{
    /// Create a new use case
    pub fn new(user_service: UserService<R>) -> Self {
        Self { user_service }
    }

    /// Get all users use case
    #[instrument(skip(self))]
    pub async fn get_users(&self, paginate_sort: &PaginateSort) -> ApiResult<GetUsersResponse> {
        self.user_service.get_users(paginate_sort).await
    }

    /// Get a user use case
    #[instrument(skip(self))]
    pub async fn get_user(&self, request: GetUserRequest) -> ApiResult<GetUserResponse> {
        validate_request_data(&request)?;

        self.user_service.get_user(request).await
    }

    /// Login use case
    // TODO: Add unit test
    #[instrument(skip(self))]
    pub async fn login(&self, request: LoginRequest, jwt: &Jwt) -> ApiResult<LoginResponse> {
        validate_request_data(&request)?;

        self.user_service.login(request, jwt).await
    }

    /// Create user use case
    // TODO: Add unit test
    #[instrument(skip(self))]
    pub async fn create_user(&self, request: CreateUserRequest) -> ApiResult<GetUserResponse> {
        validate_request_data(&request)?;

        self.user_service.create_user(request).await
    }

    /// Delete user use case
    #[instrument(skip(self))]
    pub async fn delete_user(&self, request: DeleteUserRequest) -> ApiResult<u64> {
        validate_request_data(&request)?;

        self.user_service.delete_user(request).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::user::User;
    use async_trait::async_trait;
    use chrono::{DateTime, Utc};
    use clean_architecture_shared::error::ApiError;
    use uuid::Uuid;

    // TODO: Deduplicate UserRepository implementation (idem UserService)
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

    fn init_use_case() -> UserUseCase<TestUserRepository> {
        let user_repository = TestUserRepository {};
        let service = UserService::new(user_repository);
        UserUseCase::new(service)
    }

    #[tokio::test]
    async fn test_get_user_use_case() {
        let use_case = init_use_case();
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

        assert_eq!(use_case.get_user(request).await, Ok(user));
    }

    #[tokio::test]
    async fn test_get_users_use_case() {
        let use_case = init_use_case();
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

        assert_eq!(use_case.get_users(&pagination_sort).await, Ok(users));
    }

    #[tokio::test]
    async fn test_create_user_use_case() {
        let use_case = init_use_case();
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

        assert_eq!(use_case.create_user(request).await, Ok(user));
    }

    #[tokio::test]
    async fn test_create_user_use_case_with_invalid_request() {
        let use_case = init_use_case();

        // Invalid password
        let request = CreateUserRequest {
            lastname: "Doe".to_string(),
            firstname: "John".to_string(),
            email: "john.doe@test.com".to_string(),
            password: "0000000".to_string(),
        };
        assert!(use_case.create_user(request).await.is_err());

        // Invalid email
        let request = CreateUserRequest {
            lastname: "Doe".to_string(),
            firstname: "John".to_string(),
            email: "john.doe".to_string(),
            password: "00000000".to_string(),
        };
        assert!(use_case.create_user(request).await.is_err());
    }
}
