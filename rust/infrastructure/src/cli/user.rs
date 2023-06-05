//! User module

use crate::{
    config::Config,
    database::{
        mysql::{
            repositories::{password_reset::PasswordResetMysqlRepository, user::UserMysqlRepository},
            Db,
        },
        GenericDb,
    },
    email::{Email, EmailConfig},
};
use clean_architecture_domain::{
    ports::{requests::user::CreateUserRequest, services::user::UserService},
    usecases::user::UserUseCase,
};
use clean_architecture_shared::error::{CliError, CliResult};
use std::sync::Arc;

/// Create a new user
pub async fn register(lastname: &str, firstname: &str, email: &str, password: &str) -> CliResult<()> {
    println!("\nCreating new user...");

    // Load configuration
    let config = Config::from_env().map_err(|err| CliError::ConfigError(err.to_string()))?;
    println!("    ► Configuration.....OK");

    // Database
    let db = Db::new()
        .await
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;
    println!("    ► Database..........OK");

    // User use case
    let email_service = Arc::new(Email::new(EmailConfig::from(config)));
    let user_repository = Arc::new(UserMysqlRepository::new(db.clone()));
    let password_reset_repository = Arc::new(PasswordResetMysqlRepository::new(db));
    let user_service = Arc::new(UserService::new(user_repository, password_reset_repository));
    let user_use_case = UserUseCase::new(user_service, email_service);

    user_use_case
        .create_user(CreateUserRequest {
            lastname: lastname.trim().to_string(),
            firstname: firstname.trim().to_string(),
            email: email.trim().to_string(),
            password: password.trim().to_string(),
        })
        .await
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;
    println!("\n→ User creation success");

    Ok(())
}
