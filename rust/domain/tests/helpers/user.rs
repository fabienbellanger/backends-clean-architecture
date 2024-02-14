use async_trait::async_trait;
use chrono::{DateTime, Utc};
use clean_architecture_domain::entities::scope::Scope;
use clean_architecture_domain::entities::user::{User, UserId};
use clean_architecture_domain::repositories::user::UserRepository;
use clean_architecture_domain::requests::user::{
    CreateUserRequest, DeleteUserRequest, LoginRequest, UpdateUserPasswordRepositoryRequest, UserIdRequest,
    UserScopeRequest,
};
use clean_architecture_domain::value_objects::email::Email;
use clean_architecture_domain::value_objects::password::Password;
use clean_architecture_shared::error::{ApiError, ApiResult};
use clean_architecture_shared::query_parameter::PaginateSort;
use uuid::Uuid;

pub(crate) const DATE: &str = "2023-04-01T12:10:00+00:00";
pub(crate) const JWT_SECRET: &str = "mySecretKey";
pub(crate) const USER_ID: &str = "3288fb86-db99-471d-95bc-1451c7ec6f7b";
pub(crate) const OTHER_USER_ID: &str = "3288fb86-db99-471d-95bc-1451c7ec6f7c";
pub(crate) const USER_EMAIL: &str = "test@test.com";
pub(crate) const TOTAL_USERS: i64 = 10;
pub(crate) const SCOPE_ID: &str = "users:read";
pub(crate) const SCOPES: [&str; 2] = ["users:read", "scopes:write"];

pub(crate) struct TestUserRepository {}

#[async_trait]
impl UserRepository for TestUserRepository {
    async fn get_users(&self, _paginate_sort: &PaginateSort) -> ApiResult<Vec<User>> {
        let date = DateTime::parse_from_rfc3339(DATE).unwrap().with_timezone(&Utc);

        Ok(vec![User {
            id: Uuid::parse_str(USER_ID).unwrap(),
            lastname: "Doe".to_string(),
            firstname: "John".to_string(),
            email: Email::new("john.doe@test.com").unwrap(),
            password: Password::new("00000000").unwrap(),
            created_at: date,
            updated_at: date,
            deleted_at: None,
        }])
    }

    async fn get_user_by_id(&self, request: UserIdRequest) -> ApiResult<User> {
        let id = Uuid::parse_str(USER_ID).unwrap();
        if id == request.id {
            let date = DateTime::parse_from_rfc3339(DATE).unwrap().with_timezone(&Utc);
            let user = User {
                id: Uuid::parse_str(USER_ID).unwrap(),
                lastname: "Doe".to_string(),
                firstname: "John".to_string(),
                email: Email::new("john.doe@test.com").unwrap(),
                password: Password::new("00000000").unwrap(),
                created_at: date,
                updated_at: date,
                deleted_at: None,
            };
            Ok(user)
        } else {
            Err(ApiError::NotFound {
                message: "no user found".to_owned(),
            })
        }
    }

    async fn get_user_by_email(&self, email: String) -> ApiResult<User> {
        if USER_EMAIL == &email {
            let date = DateTime::parse_from_rfc3339(DATE).unwrap().with_timezone(&Utc);
            let user = User {
                id: Uuid::parse_str(USER_ID).unwrap(),
                lastname: "Doe".to_string(),
                firstname: "John".to_string(),
                email: Email::new(USER_EMAIL).unwrap(),
                password: Password::new("00000000").unwrap(),
                created_at: date,
                updated_at: date,
                deleted_at: None,
            };
            Ok(user)
        } else {
            Err(ApiError::NotFound {
                message: "no user found".to_owned(),
            })
        }
    }

    async fn login(&self, request: LoginRequest) -> ApiResult<Option<User>> {
        let date = DateTime::parse_from_rfc3339(DATE).unwrap().with_timezone(&Utc);
        if request.email == USER_EMAIL {
            let user = User {
                id: Uuid::parse_str(USER_ID).unwrap(),
                lastname: "Doe".to_owned(),
                firstname: "John".to_owned(),
                email: Email::new(&request.email).unwrap(),
                password: Password::new(&request.password).unwrap(),
                created_at: date,
                updated_at: date,
                deleted_at: None,
            };

            return Ok(Some(user));
        }

        Ok(None)
    }

    async fn create_user(&self, request: CreateUserRequest) -> ApiResult<User> {
        let date = DateTime::parse_from_rfc3339(DATE).unwrap().with_timezone(&Utc);
        Ok(User {
            id: Uuid::parse_str(USER_ID).unwrap(),
            lastname: request.lastname,
            firstname: request.firstname,
            email: Email::new(&request.email).unwrap(),
            password: Password::new(&request.password).unwrap(),
            created_at: date,
            updated_at: date,
            deleted_at: None,
        })
    }

    async fn delete_user(&self, request: DeleteUserRequest) -> ApiResult<u64> {
        let user_id: Uuid = USER_ID.parse().unwrap();
        if user_id == request.id {
            Ok(1)
        } else {
            Ok(0)
        }
    }

    async fn get_total_users(&self) -> ApiResult<i64> {
        Ok(TOTAL_USERS)
    }

    async fn update_password(&self, _request: UpdateUserPasswordRepositoryRequest) -> ApiResult<()> {
        Ok(())
    }

    async fn get_scopes(&self, id: UserId) -> ApiResult<Vec<Scope>> {
        let user_id: Uuid = USER_ID.parse().unwrap();
        if user_id == id {
            Ok(SCOPES.iter().map(|s| Scope::new(s.to_string())).collect())
        } else {
            Ok(vec![])
        }
    }

    async fn add_scope(&self, request: UserScopeRequest) -> ApiResult<u64> {
        let user_id: Uuid = USER_ID.parse().unwrap();
        if user_id == request.user_id {
            Ok(1)
        } else {
            Err(ApiError::NotFound {
                message: "no user found".to_owned(),
            })
        }
    }

    async fn remove_scope(&self, request: UserScopeRequest) -> ApiResult<u64> {
        let user_id: Uuid = USER_ID.parse().unwrap();
        if user_id == request.user_id {
            Ok(1)
        } else {
            Err(ApiError::NotFound {
                message: "no user found".to_owned(),
            })
        }
    }
}
