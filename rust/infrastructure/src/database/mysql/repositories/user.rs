//! User MySQL repository

use crate::database::mysql::models::scope::ScopeModel;
use crate::database::mysql::models::user::UserModel;
use crate::database::mysql::Db;
use async_trait::async_trait;
use chrono::Utc;
use clean_architecture_domain::entities::scope::Scope;
use clean_architecture_domain::entities::user::UserId;
use clean_architecture_domain::requests::user::{
    CreateUserRequest, DeleteUserRequest, UpdateUserPasswordRepositoryRequest, UserScopeRequest,
};
use clean_architecture_domain::{
    entities::user::User,
    {
        repositories::user::UserRepository,
        requests::user::{LoginRequest, UserIdRequest},
    },
};
use clean_architecture_shared::error::{ApiError, ApiResult};
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
    #[instrument(skip(self), name = "user_repository_get_users")]
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

        Ok(users.into_iter().filter_map(|u| u.try_into().ok()).collect())
    }

    #[instrument(skip(self), name = "user_repository_get_user_by_id")]
    async fn get_user_by_id(&self, request: UserIdRequest) -> ApiResult<User> {
        let user = sqlx::query_as!(
            UserModel,
            "SELECT * FROM users WHERE id = ? AND deleted_at IS NULL",
            request.id.to_string()
        )
        .fetch_optional(self.db.pool.clone().as_ref())
        .await?;

        match user {
            Some(user) => user.try_into(),
            None => Err(ApiError::NotFound {
                message: "no user found".to_owned(),
            }),
        }
    }

    #[instrument(skip(self), name = "user_repository_get_user_by_email")]
    async fn get_user_by_email(&self, email: String) -> ApiResult<User> {
        let user = sqlx::query_as!(
            UserModel,
            "SELECT * FROM users WHERE email = ? AND deleted_at IS NULL",
            email
        )
        .fetch_optional(self.db.pool.clone().as_ref())
        .await?;

        match user {
            Some(user) => user.try_into(),
            None => Err(ApiError::NotFound {
                message: "no user found".to_owned(),
            }),
        }
    }

    #[instrument(skip(self), name = "user_repository_login")]
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

    #[instrument(skip(self), name = "user_repository_create_user")]
    async fn create_user(&self, request: CreateUserRequest) -> ApiResult<User> {
        let hashed_password = format!("{:x}", Sha512::digest(request.password.as_bytes()));
        let user_id = uuid::Uuid::new_v4();

        // Create user
        sqlx::query!(
            "
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
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        // Get user
        self.get_user_by_id(UserIdRequest { id: user_id }).await
    }

    #[instrument(skip(self), name = "user_repository_delete_user")]
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

    #[instrument(skip(self), name = "user_repository_get_total_users")]
    async fn get_total_users(&self) -> ApiResult<i64> {
        let result = sqlx::query!("SELECT COUNT(*) AS total FROM users WHERE deleted_at IS NULL")
            .fetch_one(self.db.pool.clone().as_ref())
            .await?;

        Ok(result.total)
    }

    #[instrument(skip(self), name = "user_repository_update_password")]
    async fn update_password(&self, request: UpdateUserPasswordRepositoryRequest) -> ApiResult<()> {
        let hashed_password = format!("{:x}", Sha512::digest(request.password.as_bytes()));

        sqlx::query!(
            r#"
                UPDATE users
                SET password = ?, updated_at = ?
                WHERE id = ?
            "#,
            hashed_password,
            Some(Utc::now()),
            request.id
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(())
    }

    #[instrument(skip(self), name = "user_repository_get_scopes")]
    // TODO: Optimize query, just ID is needed
    async fn get_scopes(&self, user_id: UserId) -> ApiResult<Vec<Scope>> {
        let scopes = sqlx::query_as!(
            ScopeModel,
            r#"
                SELECT
                    scopes.id,
                    scopes.created_at,
                    scopes.deleted_at
                FROM scopes 
                    INNER JOIN users_scopes ON scopes.id = users_scopes.scope_id
                WHERE users_scopes.user_id = ?
                    AND scopes.deleted_at IS NULL
            "#,
            user_id.to_string()
        )
        .fetch_all(self.db.pool.clone().as_ref())
        .await?;

        Ok(scopes.into_iter().map(|s| s.into()).collect())
    }

    #[instrument(skip(self), name = "user_repository_add_scope")]
    async fn add_scope(&self, request: UserScopeRequest) -> ApiResult<u64> {
        let result = sqlx::query!(
            "INSERT IGNORE INTO users_scopes (user_id, scope_id) VALUES (?, ?)",
            request.user_id.to_string(),
            request.scope_id.to_string()
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(result.rows_affected())
    }

    #[instrument(skip(self), name = "user_repository_remove_scope")]
    async fn remove_scope(&self, request: UserScopeRequest) -> ApiResult<u64> {
        let result = sqlx::query!(
            "DELETE FROM users_scopes WHERE user_id = ? AND scope_id = ?",
            request.user_id.to_string(),
            request.scope_id.to_string()
        )
        .execute(self.db.pool.clone().as_ref())
        .await?;

        Ok(result.rows_affected())
    }
}
