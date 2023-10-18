use super::{TestApp, TestResponse};
use clean_architecture_domain::{
    entities::user::User,
    ports::{repositories::user::UserRepository, requests::user::CreateUserRequest, responses::user::LoginResponse},
    value_objects::password::Password,
};
use clean_architecture_infrastructure::database::mysql::repositories::user::UserMysqlRepository;

/// Create a user for authentication
pub async fn create_user(app: &TestApp) -> User {
    let repository = UserMysqlRepository::new(app.database.database());

    // Create a new user
    let password = String::from("00000000");
    let request = CreateUserRequest {
        lastname: "Doe".to_string(),
        firstname: "John".to_string(),
        email: "john.doe@test.com".to_string(),
        password: password.clone(),
    };
    let mut user = repository.create_user(request.clone()).await.unwrap();
    user.password = Password::new(&password).unwrap();

    user
}

/// Create, authenticate a user and return `TestResponse` and the generated JWT
pub async fn create_and_authenticate(app: &TestApp) -> (TestResponse, String) {
    let user = create_user(app).await;

    let response = login_request(
        &app,
        serde_json::json!({
            "email": user.email.value(),
            "password": user.password.value()
        })
        .to_string(),
    )
    .await;

    let res: LoginResponse = serde_json::from_str(&response.body.to_string()).expect("error when deserializing body");

    (response, res.access_token)
}

/// Login request helper
pub async fn login_request(app: &TestApp, body: String) -> TestResponse {
    TestResponse::new(app, "/api/v1/login", "POST", Some(body), None).await
}

/// User creation request helper
pub async fn create_user_request(app: &TestApp, body: String, token: &str) -> TestResponse {
    TestResponse::new(app, "/api/v1/users", "POST", Some(body), Some(token)).await
}

/// Return all users
pub async fn get_all(app: &TestApp, token: &str) -> TestResponse {
    TestResponse::new(app, "/api/v1/users", "GET", None, Some(token)).await
}

/// Return a user
pub async fn get_one(app: &TestApp, token: &str, id: &str) -> TestResponse {
    TestResponse::new(app, &format!("/api/v1/users/{id}"), "GET", None, Some(token)).await
}

/// Delete a user
pub async fn delete(app: &TestApp, token: &str, id: &str) -> TestResponse {
    TestResponse::new(app, &format!("/api/v1/users/{id}"), "DELETE", None, Some(token)).await
}
