//! User MySQL repository module

use async_trait::async_trait;
use clean_architecture_domain::{
    entities::{error::UserError, user::User},
    ports::{
        repositories::user::UserRepository,
        requests::user::{GetUserRequest, LoginRequest},
    },
};
use tracing::instrument;

#[derive(Debug)]
pub struct UserMysqlRepository {
    _pool: sqlx::mysql::MySqlPool,
}

#[async_trait]
impl UserRepository for UserMysqlRepository {
    #[instrument(skip(self))]
    async fn get_users(&self) -> Result<Vec<User>, UserError> {
        todo!()
    }

    #[instrument(skip(self))]
    async fn get_user(&self, request: GetUserRequest) -> Result<User, UserError> {
        dbg!(&request);
        todo!()
    }

    #[instrument(skip(self))]
    async fn login(&self, request: LoginRequest) -> Result<User, UserError> {
        dbg!(&request);
        todo!()
    }
}
