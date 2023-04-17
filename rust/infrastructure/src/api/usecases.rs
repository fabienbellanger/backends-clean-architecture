//! Use cases module

use crate::database::mysql::{repositories::user::UserMysqlRepository, Db};
use crate::email::Email;
use clean_architecture_domain::{ports::services::user::UserService, usecases::user::UserUseCase};
use clean_architecture_shared::error::ApiResult;

#[derive(Clone)]
pub struct AppUseCases {
    pub user: UserUseCase<UserMysqlRepository, Email>,
}

impl AppUseCases {
    pub async fn new(db: Db, email: Email) -> ApiResult<Self> {
        // User
        let user_repository = UserMysqlRepository::new(db);
        let user_service = UserService::new(user_repository);
        let user_use_case = UserUseCase::new(user_service, email);

        Ok(Self {
            user: user_use_case,
        })
    }
}
