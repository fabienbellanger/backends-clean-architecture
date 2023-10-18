//! Use cases module

use crate::database::mysql::repositories::password_reset::PasswordResetMysqlRepository;
use crate::database::mysql::repositories::refresh_token::RefreshTokenMysqlRepository;
use crate::database::mysql::{repositories::user::UserMysqlRepository, Db};
use crate::email::Email;
use clean_architecture_domain::{ports::services::user::UserService, usecases::user::UserUseCase};
use clean_architecture_shared::error::ApiResult;

#[derive(Clone)]
pub struct AppUseCases {
    pub user: UserUseCase<UserMysqlRepository, PasswordResetMysqlRepository, RefreshTokenMysqlRepository, Email>,
}

impl AppUseCases {
    pub async fn new(db: Db, email: Email) -> ApiResult<Self> {
        // User
        let user_repository = UserMysqlRepository::new(db.clone());
        let password_reset_repository = PasswordResetMysqlRepository::new(db.clone());
        let refresh_token_repository = RefreshTokenMysqlRepository::new(db);
        let user_service = UserService::new(user_repository, password_reset_repository, refresh_token_repository);
        let user_use_case = UserUseCase::new(user_service, email);

        Ok(Self { user: user_use_case })
    }
}
