use crate::helpers::api::axum_rest::user::{add_scope, get_scopes, remove_scope};
use crate::helpers::api::axum_rest::{
    user::{create_user_request, delete, get_all, get_one, login_request},
    TestApp, TestAppBuilder,
};
use axum::http::StatusCode;
use clean_architecture_domain::entities::scope::{ScopeId, SCOPE_USERS};
use clean_architecture_domain::responses::user::{GetUserResponse, GetUsersResponse};

#[tokio::test]
async fn test_api_login_unauthorized_user() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let response = login_request(
        &app,
        serde_json::json!({
            "email": "test@gmail.com",
            "password": "00000000"
        })
        .to_string(),
    )
    .await;

    assert_eq!(response.status_code, StatusCode::UNAUTHORIZED);
    assert_eq!(
        response.body,
        serde_json::json!({
            "code": 401,
            "message": "Unauthorized"
        })
    );
}

#[tokio::test]
async fn test_api_login_authorized_user() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (response, _token) = app.make_authentication().await;

    assert_eq!(response.status_code, StatusCode::OK);
}

#[tokio::test]
async fn test_api_user_creation_success() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    let response = create_user_request(
        &app,
        serde_json::json!({
            "email": "test-user-creation@test.com",
            "password": "00000000",
            "lastname": "Test",
            "firstname": "Toto",
        })
        .to_string(),
        &token,
    )
    .await;

    assert_eq!(response.status_code, StatusCode::OK);
}

#[tokio::test]
async fn test_api_user_creation_invalid_password() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    let response = create_user_request(
        &app,
        serde_json::json!({
            "email": "test-user-creation@test.com",
            "password": "0000000",
            "lastname": "Test",
            "firstname": "Toto",
        })
        .to_string(),
        &token,
    )
    .await;

    assert_eq!(response.status_code, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_api_user_list_all() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    // Create 2 users
    for i in 1..3 {
        create_user_request(
            &app,
            serde_json::json!({
                "email": format!("test-user-creation-{i}@test.com"),
                "password": "00000000",
                "lastname": "Test",
                "firstname": format!("Toto {i}"),
                "rate_limit": 10,
            })
            .to_string(),
            &token,
        )
        .await;
    }

    let response = get_all(&app, &token).await;
    assert_eq!(response.status_code, StatusCode::OK);

    let users: GetUsersResponse =
        serde_json::from_str(&response.body.to_string()).expect("error when deserializing body");
    assert_eq!(users.data.len(), 3);
    assert_eq!(users.total, 3);
}

#[tokio::test]
async fn test_api_user_list_one() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    // Create a user
    let response = create_user_request(
        &app,
        serde_json::json!({
            "email": "test-user-creation@test.com",
            "password": "00000000",
            "lastname": "Test",
            "firstname": "Toto",
        })
        .to_string(),
        &token,
    )
    .await;

    let user: GetUserResponse =
        serde_json::from_str(&response.body.to_string()).expect("error when deserializing body");
    let response = get_one(&app, &token, &user.id).await;
    assert_eq!(response.status_code, StatusCode::OK);

    let expected_user: GetUserResponse =
        serde_json::from_str(&response.body.to_string()).expect("error when deserializing body");
    assert_eq!(expected_user.id, user.id);
}

#[tokio::test]
async fn test_api_user_get_one_bad_parameter() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    let response = get_one(&app, &token, "bad_id").await;
    assert_eq!(response.status_code, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_api_user_delete() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    // Create a user
    let response = create_user_request(
        &app,
        serde_json::json!({
            "email": "test-user-creation@test.com",
            "password": "00000000",
            "lastname": "Test",
            "firstname": "Toto",
        })
        .to_string(),
        &token,
    )
    .await;

    let user: GetUserResponse =
        serde_json::from_str(&response.body.to_string()).expect("error when deserializing body");
    let response = delete(&app, &token, &user.id).await;
    assert_eq!(response.status_code, StatusCode::NO_CONTENT);

    let response = get_one(&app, &token, &user.id).await;
    assert_eq!(response.status_code, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_api_user_add_scope() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    // Create a user
    let response = create_user_request(
        &app,
        serde_json::json!({
            "email": "test-user-creation@test.com",
            "password": "00000000",
            "lastname": "Test",
            "firstname": "Toto",
        })
        .to_string(),
        &token,
    )
    .await;
    let user: GetUserResponse =
        serde_json::from_str(&response.body.to_string()).expect("error when deserializing body");

    let response = add_scope(
        &app,
        &token,
        &user.id,
        serde_json::json!({
            "id": SCOPE_USERS.to_string(),
        })
        .to_string(),
    )
    .await;
    assert_eq!(response.status_code, StatusCode::CREATED);
}

#[tokio::test]
async fn test_api_user_remove_scope() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    // Create a user
    let response = create_user_request(
        &app,
        serde_json::json!({
            "email": "test-user-creation@test.com",
            "password": "00000000",
            "lastname": "Test",
            "firstname": "Toto",
        })
        .to_string(),
        &token,
    )
    .await;
    let user: GetUserResponse =
        serde_json::from_str(&response.body.to_string()).expect("error when deserializing body");

    // Add scope
    add_scope(
        &app,
        &token,
        &user.id,
        serde_json::json!({
            "id": SCOPE_USERS.to_string(),
        })
        .to_string(),
    )
    .await;

    // Remove scope
    let response = remove_scope(&app, &token, &user.id, SCOPE_USERS).await;
    assert_eq!(response.status_code, StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn test_api_user_get_scopes() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    // Create a user
    let response = create_user_request(
        &app,
        serde_json::json!({
            "email": "test-user-creation@test.com",
            "password": "00000000",
            "lastname": "Test",
            "firstname": "Toto",
        })
        .to_string(),
        &token,
    )
    .await;
    let user: GetUserResponse =
        serde_json::from_str(&response.body.to_string()).expect("error when deserializing body");

    // Add scope
    add_scope(
        &app,
        &token,
        &user.id,
        serde_json::json!({
            "id": SCOPE_USERS.to_string(),
        })
        .to_string(),
    )
    .await;

    // Get scopes
    let response = get_scopes(&app, &token, &user.id).await;
    assert_eq!(response.status_code, StatusCode::OK);

    let scopes: Vec<ScopeId> = serde_json::from_str(&response.body.to_string()).expect("error when deserializing body");
    assert_eq!(scopes, vec![SCOPE_USERS]);
}
