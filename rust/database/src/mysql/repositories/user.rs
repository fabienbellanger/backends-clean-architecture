//! User MySQL repository module
//!
//!```ignore
//! use clean_architecture_database::{
//!     init_mysql_pool, mysql::repositories::user::UserMysqlRepository,
//! };
//! use clean_architecture_domain::ports::{
//!     repositories::user::UserRepository,
//!     requests::user::{CreateUserRequest, GetUserRequest, LoginRequest},
//! };
//! use clean_architecture_shared::error::ApiResult;
//! use clean_architecture_shared::query_parameter::PaginateSort;
//!
//! #[tokio::main]
//! async fn main() -> ApiResult<()> {
//!     let pool = init_mysql_pool().await?;
//!     let user_repository = UserMysqlRepository::new(&pool);
//!
//!     // Get users
//!     let pagination_sort = PaginateSort {
//!         page: 1,
//!         limit: 10,
//!         offset: 0,
//!         sorts: vec![],
//!     };
//!     let users = user_repository.get_users(&pagination_sort).await?;
//!     dbg!(users);
//!
//!     // Get one user
//!     let user = user_repository
//!         .get_user(GetUserRequest {
//!             id: uuid::uuid!("262b7a80-4304-4979-ac20-0f828fe275fe"),
//!         })
//!         .await?;
//!     dbg!(user);
//!
//!     // User login
//!     let login = user_repository
//!         .login(LoginRequest {
//!             email: String::from("toto@apitic.com"),
//!             password: String::from("00000000"),
//!         })
//!         .await?;
//!     dbg!(login);
//!
//!     // Create new user
//!     let new_user = CreateUserRequest {
//!         email: "test@testest.com".to_owned(),
//!         password: "11111111".to_owned(),
//!         lastname: "Doe".to_owned(),
//!         firstname: "Jane".to_owned(),
//!     };
//!     let user = user_repository.create_user(new_user).await?;
//!     dbg!(user);
//!
//!     Ok(())
//! }
//! ```
use crate::mysql::models::user::UserModel;
use crate::mysql::Db;
use async_trait::async_trait;
use chrono::Utc;
use clean_architecture_domain::ports::requests::user::{CreateUserRequest, DeleteUserRequest};
use clean_architecture_domain::{
    entities::user::User,
    ports::{
        repositories::user::UserRepository,
        requests::user::{GetUserRequest, LoginRequest},
    },
};
use clean_architecture_shared::error::ApiResult;
use clean_architecture_shared::query_parameter::PaginateSort;
use sha2::{Digest, Sha512};
use std::sync::Arc;
use tracing::instrument;

/// User MySQL repository
#[derive(Debug, Clone)]
pub struct UserMysqlRepository {
    db: Arc<Db>,
}

impl UserMysqlRepository {
    /// Create a new repository
    pub fn new(db: Db) -> Self {
        Self { db: Arc::new(db) }
    }
}

#[async_trait]
impl UserRepository for UserMysqlRepository {
    #[instrument(skip(self))]
    async fn get_users(&self, paginate_sort: &PaginateSort) -> ApiResult<Vec<User>> {
        let mut query = String::from(
            "
            SELECT * 
            FROM users 
            WHERE deleted_at IS NULL
            ",
        );

        // Sorts and pagination
        query.push_str(&paginate_sort.get_sorts_sql(Some(&[
            "id",
            "lastname",
            "firstname",
            "created_at",
            "updated_at",
            "deleted_at",
        ])));
        query.push_str(&paginate_sort.get_pagination_sql());

        let users = sqlx::query_as::<_, UserModel>(&query)
            .fetch_all(self.db.pool.clone().as_ref())
            .await?;

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
        .fetch_one(self.db.pool.clone().as_ref())
        .await?;

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
        .fetch_optional(self.db.pool.clone().as_ref())
        .await?;

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
        ).execute(self.db.pool.clone().as_ref())
            .await?;

        // Get user
        self.get_user(GetUserRequest { id: user_id }).await
    }

    #[instrument(skip(self))]
    async fn delete_user(&self, request: DeleteUserRequest) -> ApiResult<u64> {
        let result = sqlx::query!(
            "UPDATE users SET deleted_at = ? WHERE id = ? AND deleted_at IS NULL",
            Some(Utc::now()),
            request.id.to_string()
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(result.rows_affected())
    }

    #[instrument(skip(self))]
    async fn get_total_users(&self) -> ApiResult<i64> {
        let result = sqlx::query!("SELECT COUNT(*) AS total FROM users WHERE deleted_at IS NULL")
            .fetch_one(self.db.pool.clone().as_ref())
            .await?;

        Ok(result.total)
    }
}
