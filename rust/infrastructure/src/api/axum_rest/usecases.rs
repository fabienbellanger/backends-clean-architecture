//! Use cases module

use crate::database::mysql::repositories::password_reset::PasswordResetMysqlRepository;
use crate::database::mysql::{repositories::user::UserMysqlRepository, Db};
use crate::email::Email;
use clean_architecture_domain::{ports::services::user::UserService, usecases::user::UserUseCase};
use clean_architecture_shared::error::ApiResult;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppUseCases {
    pub user: UserUseCase<UserMysqlRepository, PasswordResetMysqlRepository, Email>,
}

impl AppUseCases {
    pub async fn new(db: Db, email: Arc<Email>) -> ApiResult<Self> {
        // User
        let user_repository = Arc::new(UserMysqlRepository::new(db.clone()));
        let password_reset_repository = Arc::new(PasswordResetMysqlRepository::new(db));
        let user_service = Arc::new(UserService::new(user_repository, password_reset_repository));
        let user_use_case = UserUseCase::new(user_service, email);

        Ok(Self { user: user_use_case })
    }
}
