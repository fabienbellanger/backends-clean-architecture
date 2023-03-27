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
//!     let users = user_repository.get_users().await.unwrap();
//!     dbg!(users);
//!
//!     let user = user_repository
//!         .get_user(GetUserRequest {
//!             id: uuid::uuid!("262b7a80-4304-4979-ac20-0f828fe275fe"),
//!         })
//!         .await
//!         .unwrap();
//!     dbg!(user);
//!
//!     let login = user_repository
//!         .login(LoginRequest {
//!             email: String::from("toto@apitic.com"),
//!             password: String::from("00000000"),
//!         })
//!         .await
//!         .unwrap();
//!     dbg!(login);
//!
//!     Ok(())
//! }
//! ```
use crate::mysql::models::user::UserModel;
use async_trait::async_trait;
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

#[derive(Debug)]
pub struct UserMysqlRepository<'a> {
    pool: &'a MySqlPool,
}

impl<'a> UserMysqlRepository<'a> {
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
    async fn login(&self, request: LoginRequest) -> ApiResult<User> {
        let hashed_password = format!("{:x}", Sha512::digest(request.password.as_bytes()));
        let user = sqlx::query_as!(
            UserModel,
            "SELECT * FROM users WHERE email = ? AND password = ?",
            request.email,
            hashed_password
        )
        .fetch_one(self.pool)
        .await
        .map_err(|err| api_error!(ApiErrorCode::InternalError, err))?;

        user.try_into()
    }

    #[instrument(skip(self))]
    async fn create_user(&self, request: CreateUserRequest) -> ApiResult<User> {
        todo!()
    }
}
