use crate::helpers::scope::{TestScopeRepository, SCOPE_ID};
use crate::helpers::user::DATE;
use clean_architecture_domain::ports::requests::scope::{CreateRequest, DeleteRequest};
use clean_architecture_domain::ports::responses::scope::ScopeResponse;
use clean_architecture_domain::ports::services::scope::ScopeService;
use clean_architecture_domain::usecases::scope::ScopeUseCase;

fn init_use_case() -> ScopeUseCase<TestScopeRepository> {
    let scope_repository = TestScopeRepository {};
    let scope_service = ScopeService::new(scope_repository);
    ScopeUseCase::new(scope_service)
}

#[tokio::test]
async fn test_create_scope_use_case() {
    let use_case = init_use_case();
    let request = CreateRequest {
        id: SCOPE_ID.to_string(),
    };

    assert!(use_case.create(request).await.is_ok());
}

#[tokio::test]
async fn test_get_scopes_use_case() {
    let use_case = init_use_case();
    let scopes: Vec<ScopeResponse> = vec![ScopeResponse {
        id: SCOPE_ID.to_string(),
        created_at: DATE.to_string(),
    }];

    assert_eq!(use_case.get_scopes().await, Ok(scopes));
}

#[tokio::test]
async fn test_delete_scope_use_case_success() {
    let use_case = init_use_case();
    let request = DeleteRequest {
        id: SCOPE_ID.to_string(),
    };

    assert_eq!(use_case.delete(request).await, Ok(1));
}

#[tokio::test]
async fn test_delete_scope_use_case_not_found() {
    let use_case = init_use_case();
    let request = DeleteRequest { id: "test".to_string() };

    assert!(use_case.delete(request).await.is_err());
}
