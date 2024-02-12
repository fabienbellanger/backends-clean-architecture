//! Use cases module

use crate::database::mysql::repositories::password_reset::PasswordResetMysqlRepository;
use crate::database::mysql::repositories::refresh_token::RefreshTokenMysqlRepository;
use crate::database::mysql::repositories::scope::ScopeMysqlRepository;
use crate::database::mysql::{repositories::user::UserMysqlRepository, Db};
use crate::email::Email;
use clean_architecture_domain::ports::services::scope::ScopeService;
use clean_architecture_domain::usecases::scope::ScopeUseCase;
use clean_architecture_domain::{ports::services::user::UserService, usecases::user::UserUseCase};
use clean_architecture_shared::error::ApiResult;

#[derive(Clone)]
pub struct AppUseCases {
    pub user: UserUseCase<UserMysqlRepository, PasswordResetMysqlRepository, RefreshTokenMysqlRepository, Email>,
    pub scope: ScopeUseCase<ScopeMysqlRepository>,
}

impl AppUseCases {
    pub async fn new(db: Db, email: Email) -> ApiResult<Self> {
        // User
        let user_repository = UserMysqlRepository::new(db.clone());
        let password_reset_repository = PasswordResetMysqlRepository::new(db.clone());
        let refresh_token_repository = RefreshTokenMysqlRepository::new(db.clone());
        let user_service = UserService::new(user_repository, password_reset_repository, refresh_token_repository);
        let user_use_case = UserUseCase::new(user_service, email);

        // Scope
        let scope_repository = ScopeMysqlRepository::new(db);
        let scope_service = ScopeService::new(scope_repository);
        let scope_use_case = ScopeUseCase::new(scope_service);

        Ok(Self {
            user: user_use_case,
            scope: scope_use_case,
        })
    }
}
