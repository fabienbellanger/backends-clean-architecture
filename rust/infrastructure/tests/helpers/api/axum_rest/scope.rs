use crate::helpers::api::axum_rest::{TestApp, TestResponse};

/// Scope creation request helper
pub async fn create_scope_request(app: &TestApp, body: String, token: &str) -> TestResponse {
    TestResponse::new(app, "/api/v1/scopes", "POST", Some(body), Some(token)).await
}

/// Scopes list request helper
pub async fn get_scopes_request(app: &TestApp, token: &str) -> TestResponse {
    TestResponse::new(app, "/api/v1/scopes", "GET", None, Some(token)).await
}

/// Scope deletion request helper
pub async fn delete_scope_request(app: &TestApp, token: &str, id: &str) -> TestResponse {
    TestResponse::new(app, "/api/v1/scopes/{id}", "DELETE", None, Some(token)).await
}
