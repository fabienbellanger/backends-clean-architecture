use crate::helpers::api::axum_rest::scope::{create_scope_request, delete_scope_request, get_scopes_request};
use crate::helpers::api::axum_rest::{TestApp, TestAppBuilder};
use axum::http::StatusCode;
use clean_architecture_domain::ports::responses::scope::ScopeResponse;

#[tokio::test]
async fn test_api_scope_creation_success() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    let response = create_scope_request(
        &app,
        serde_json::json!({
            "id": "scope"
        })
        .to_string(),
        &token,
    )
    .await;

    assert_eq!(response.status_code, StatusCode::CREATED);
}

#[tokio::test]
async fn test_api_scope_creation_invalid_id() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    let response = create_scope_request(
        &app,
        serde_json::json!({
            "id": "sss"
        })
        .to_string(),
        &token,
    )
    .await;

    assert_eq!(response.status_code, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_api_get_scopes() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    let response = get_scopes_request(&app, &token).await;
    assert_eq!(response.status_code, StatusCode::OK);

    let scopes: Vec<ScopeResponse> =
        serde_json::from_str(&response.body.to_string()).expect("error when deserializing body");
    assert_eq!(scopes.len(), 2);
}

#[tokio::test]
async fn test_api_delete_scope() {
    let app: TestApp = TestAppBuilder::new().await.build();
    let (_response, token) = app.make_authentication().await;

    // Create new scope
    create_scope_request(
        &app,
        serde_json::json!({
            "id": "scope"
        })
        .to_string(),
        &token,
    )
    .await;

    let response = delete_scope_request(&app, &token, "scope").await;
    assert_eq!(response.status_code, StatusCode::NO_CONTENT);
}
