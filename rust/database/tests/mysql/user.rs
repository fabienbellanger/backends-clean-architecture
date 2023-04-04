use super::TestMySQL;
use clean_architecture_database::mysql::repositories::user::UserMysqlRepository;
use clean_architecture_domain::ports::{
    repositories::user::UserRepository,
    requests::user::{CreateUserRequest, GetUserRequest},
};

#[tokio::test]
async fn test_create_user() {
    let db = TestMySQL::new().await;
    let repository = UserMysqlRepository::new(db.database());

    // Create a new user
    let request = CreateUserRequest {
        lastname: "Doe".to_string(),
        firstname: "John".to_string(),
        email: "john.doe@test.com".to_string(),
        password: "00000000".to_string(),
    };
    let result = repository.create_user(request.clone()).await;

    assert!(result.is_ok());
    if let Ok(user) = result {
        assert_eq!(user.lastname, request.lastname);
        assert_eq!(user.firstname, request.firstname);
        assert_eq!(user.email, request.email);
    }
}

#[tokio::test]
async fn test_create_user_with_email_already_exists() {
    let db = TestMySQL::new().await;
    let repository = UserMysqlRepository::new(db.database());

    // Create a new user
    let request = CreateUserRequest {
        lastname: "Doe".to_string(),
        firstname: "John".to_string(),
        email: "john.doe@test.com".to_string(),
        password: "00000000".to_string(),
    };
    repository.create_user(request.clone()).await.unwrap();

    // Create a new user with same email
    let request = CreateUserRequest {
        lastname: "Doe".to_string(),
        firstname: "Jessy".to_string(),
        email: "john.doe@test.com".to_string(),
        password: "00000000".to_string(),
    };
    let result = repository.create_user(request.clone()).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_user() {
    let db = TestMySQL::new().await;
    let repository = UserMysqlRepository::new(db.database());

    // Create a new user
    let request = CreateUserRequest {
        lastname: "Doe".to_string(),
        firstname: "John".to_string(),
        email: "john.doe@test.com".to_string(),
        password: "00000000".to_string(),
    };
    let user = repository.create_user(request.clone()).await.unwrap();

    // Retrieve this new user by id
    let request = GetUserRequest { id: user.id };
    let result = repository.get_user(request.clone()).await;

    assert!(result.is_ok());
}
