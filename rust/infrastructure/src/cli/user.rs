//! User

use crate::database::mysql::repositories::refresh_token::RefreshTokenMysqlRepository;
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
    use_cases::user::UserUseCase,
    {requests::user::CreateUserRequest, services::user::UserService},
};
use clean_architecture_shared::error::{CliError, CliResult};

/// Create a new user
pub async fn register(
    lastname: &str,
    firstname: &str,
    email: &str,
    password: &str,
    scopes: &Option<Vec<String>>,
) -> CliResult<()> {
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
    let email_service = Email::new(EmailConfig::from(config));
    let user_repository = UserMysqlRepository::new(db.clone());
    let password_reset_repository = PasswordResetMysqlRepository::new(db.clone());
    let refresh_token_repository = RefreshTokenMysqlRepository::new(db);
    let user_service = UserService::new(user_repository, password_reset_repository, refresh_token_repository);
    let user_use_case = UserUseCase::new(user_service, email_service);

    user_use_case
        .create_user(CreateUserRequest {
            lastname: lastname.trim().to_string(),
            firstname: firstname.trim().to_string(),
            email: email.trim().to_string(),
            password: password.trim().to_string(),
            scopes: scopes.clone(),
        })
        .await
        .map_err(|err| CliError::DatabaseError(err.to_string()))?;
    println!("\n→ User creation success");

    Ok(())
}
