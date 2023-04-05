//! Use cases module

use crate::database::mysql::{repositories::user::UserMysqlRepository, Db};
use clean_architecture_domain::{ports::services::user::UserService, usecases::user::UserUseCase};
use clean_architecture_shared::error::ApiResult;

#[derive(Clone)]
pub struct AppUseCases {
    pub user: UserUseCase<UserMysqlRepository>,
}

impl AppUseCases {
    pub async fn new(db: Db) -> ApiResult<Self> {
        // User
        let user_repository = UserMysqlRepository::new(db);
        let user_service = UserService::new(user_repository);
        let user_use_case = UserUseCase::new(user_service);

        Ok(Self {
            user: user_use_case,
        })
    }
}
