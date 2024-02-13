use crate::helpers::api::axum_rest::{TestApp, TestResponse};
use clean_architecture_domain::entities::scope::{SCOPE_ADMIN, SCOPE_USERS};
use clean_architecture_domain::ports::repositories::scope::ScopeRepository;
use clean_architecture_domain::ports::requests::scope::CreateRequest;
use clean_architecture_infrastructure::database::mysql::repositories::scope::ScopeMysqlRepository;

/// Create default scopes
pub async fn create_scopes(app: &TestApp) {
    let repository = ScopeMysqlRepository::new(app.database.database());

    let request = CreateRequest {
        id: SCOPE_ADMIN.to_string(),
    };
    repository.create(request.clone()).await.unwrap();

    let request = CreateRequest {
        id: SCOPE_USERS.to_string(),
    };
    repository.create(request.clone()).await.unwrap();
}

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
    TestResponse::new(app, &format!("/api/v1/scopes/{id}"), "DELETE", None, Some(token)).await
}
