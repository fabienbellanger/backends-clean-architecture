use clean_architecture_database::{
    init_mysql_pool, mysql::repositories::user::UserMysqlRepository,
};
use clean_architecture_domain::ports::requests::user::CreateUserRequest;
use clean_architecture_domain::ports::{
    repositories::user::UserRepository,
    requests::user::{GetUserRequest, LoginRequest},
};
use clean_architecture_shared::error::ApiResult;

#[tokio::main]
async fn main() -> ApiResult<()> {
    let pool = init_mysql_pool().await?;
    let user_repository = UserMysqlRepository::new(&pool);

    let users = user_repository.get_users().await.unwrap();
    dbg!(users);

    let user = user_repository
        .get_user(GetUserRequest {
            id: uuid::uuid!("262b7a80-4304-4979-ac20-0f828fe275fe"),
        })
        .await
        .unwrap();
    dbg!(user);

    let login = user_repository
        .login(LoginRequest {
            email: String::from("toto@apitic.com"),
            password: String::from("00000000"),
        })
        .await
        .unwrap();
    dbg!(login);

    let new_user = CreateUserRequest {
        email: "test@testest.com".to_owned(),
        password: "11111111".to_owned(),
        lastname: "Toto".to_owned(),
        firstname: "Titi".to_owned(),
    };
    let user = user_repository.create_user(new_user).await.unwrap();
    dbg!(user);

    Ok(())
}
