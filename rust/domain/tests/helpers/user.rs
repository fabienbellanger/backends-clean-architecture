use async_trait::async_trait;
use chrono::{DateTime, Utc};
use clean_architecture_domain::entities::user::User;
use clean_architecture_domain::ports::repositories::user::UserRepository;
use clean_architecture_domain::ports::requests::user::{
    CreateUserRequest, DeleteUserRequest, ForgottenPasswordRequest, GetUserRequest, LoginRequest,
};
use clean_architecture_shared::error::{ApiError, ApiResult};
use clean_architecture_shared::query_parameter::PaginateSort;
use uuid::Uuid;

pub(crate) const DATE: &str = "2023-04-01T12:10:00+00:00";
pub(crate) const JWT_SECRET: &str = "mySecretKey";
pub(crate) const USER_ID: &str = "3288fb86-db99-471d-95bc-1451c7ec6f7b";
pub(crate) const USER_EMAIL: &str = "test@test.com";
pub(crate) const TOTAL_USERS: i64 = 10;

pub(crate) struct TestUserRepository {}

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

    async fn get_user_by_id(&self, request: GetUserRequest) -> ApiResult<User> {
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

    async fn get_user_by_email(&self, email: String) -> ApiResult<User> {
        if USER_EMAIL == &email {
            let date = DateTime::parse_from_rfc3339(DATE)
                .unwrap()
                .with_timezone(&Utc);
            let user = User {
                id: Uuid::parse_str(USER_ID).unwrap(),
                lastname: "Doe".to_string(),
                firstname: "John".to_string(),
                email: USER_EMAIL.to_string(),
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

    async fn login(&self, request: LoginRequest) -> ApiResult<Option<User>> {
        let date = DateTime::parse_from_rfc3339(DATE)
            .unwrap()
            .with_timezone(&Utc);
        if request.email == USER_EMAIL {
            let user = User {
                id: Uuid::parse_str(USER_ID).unwrap(),
                lastname: "Doe".to_owned(),
                firstname: "John".to_owned(),
                email: request.email,
                password: request.password,
                created_at: date,
                updated_at: date,
                deleted_at: None,
            };

            return Ok(Some(user));
        }

        Ok(None)
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
        Ok(1)
    }

    async fn get_total_users(&self) -> ApiResult<i64> {
        Ok(TOTAL_USERS)
    }
}
