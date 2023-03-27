//! User MySQL repository module
//!
//!```ignore
//! use clean_architecture_database::{
//!     init_mysql_pool, mysql::repositories::user::UserMysqlRepository,
//! };
//! use clean_architecture_domain::ports::{
//!     repositories::user::UserRepository,
//!     requests::user::{GetUserRequest, LoginRequest},
//! };
//! use clean_architecture_shared::error::ApiResult;
//!
//! #[tokio::main]
//! async fn main() -> ApiResult<()> {
//!     let pool = init_mysql_pool().await?;
//!     let user_repository = UserMysqlRepository::new(&pool);
//!
//!     // Get users
//!     let users = user_repository.get_users().await.unwrap();
//!     dbg!(users);
//!
//!     // Get one user
//!     let user = user_repository
//!         .get_user(GetUserRequest {
//!             id: uuid::uuid!("262b7a80-4304-4979-ac20-0f828fe275fe"),
//!         })
//!         .await
//!         .unwrap();
//!     dbg!(user);
//!
//!     // User login
//!     let login = user_repository
//!         .login(LoginRequest {
//!             email: String::from("toto@apitic.com"),
//!             password: String::from("00000000"),
//!         })
//!         .await
//!         .unwrap();
//!     dbg!(login);
//!
//!     // Create new user
//!     let new_user = CreateUserRequest {
//!         email: "test@testest.com".to_owned(),
//!         password: "11111111".to_owned(),
//!         lastname: "Doe".to_owned(),
//!         firstname: "Jane".to_owned(),
//!     };
//!     let user = user_repository.create_user(new_user).await.unwrap();
//!     dbg!(user);
//!
//!     Ok(())
//! }
//! ```
use crate::mysql::models::user::UserModel;
use async_trait::async_trait;
use chrono::Utc;
use clean_architecture_domain::ports::requests::user::CreateUserRequest;
use clean_architecture_domain::{
    entities::user::User,
    ports::{
        repositories::user::UserRepository,
        requests::user::{GetUserRequest, LoginRequest},
    },
};
use clean_architecture_shared::{
    api_error,
    error::{ApiError, ApiErrorCode, ApiResult},
};
use sha2::{Digest, Sha512};
use sqlx::mysql::MySqlPool;
use tracing::instrument;

/// User MySQL repository
#[derive(Debug)]
pub struct UserMysqlRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> UserMysqlRepository<'a> {
    /// Create a new repository
    pub fn new(pool: &'a MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl<'a> UserRepository for UserMysqlRepository<'a> {
    #[instrument(skip(self))]
    async fn get_users(&self) -> ApiResult<Vec<User>> {
        let users = sqlx::query_as!(UserModel, "SELECT * FROM users WHERE deleted_at IS NULL")
            .fetch_all(self.pool)
            .await
            .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?;

        Ok(users
            .into_iter()
            .filter_map(|u| u.try_into().ok())
            .collect())
    }

    #[instrument(skip(self))]
    async fn get_user(&self, request: GetUserRequest) -> ApiResult<User> {
        let user = sqlx::query_as!(
            UserModel,
            "SELECT * FROM users WHERE id = ?",
            request.id.to_string()
        )
        .fetch_one(self.pool)
        .await
        .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?;

        user.try_into()
    }

    #[instrument(skip(self))]
    async fn login(&self, request: LoginRequest) -> ApiResult<Option<User>> {
        let hashed_password = format!("{:x}", Sha512::digest(request.password.as_bytes()));
        let user = sqlx::query_as!(
            UserModel,
            "SELECT * FROM users WHERE email = ? AND password = ?",
            request.email,
            hashed_password
        )
        .fetch_optional(self.pool)
        .await
        .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?;

        match user {
            Some(user) => Ok(Some(user.try_into()?)),
            None => Ok(None),
        }
    }

    #[instrument(skip(self))]
    async fn create_user(&self, request: CreateUserRequest) -> ApiResult<User> {
        let hashed_password = format!("{:x}", Sha512::digest(request.password.as_bytes()));
        let user_id = uuid::Uuid::new_v4();

        // Create user
        sqlx::query!("
            INSERT INTO users (id, email, password, lastname, firstname, created_at, updated_at, deleted_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, NULL)
        ",
            user_id.clone().to_string(),
            request.email,
            hashed_password,
            request.lastname,
            request.firstname,
            Utc::now(),
            Utc::now()
        ).execute(self.pool)
            .await
            .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?;

        // Get user
        self.get_user(GetUserRequest { id: user_id }).await
    }
}
