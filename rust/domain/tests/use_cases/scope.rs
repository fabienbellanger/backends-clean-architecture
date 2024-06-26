use crate::helpers::scope::{TestScopeRepository, SCOPE_ID};
use crate::helpers::user::DATE;
use chrono::{DateTime, Utc};
use clean_architecture_domain::services::scope::ScopeService;
use clean_architecture_domain::use_cases::scope::request::{CreateScopeUseCaseRequest, DeleteScopeUseCaseRequest};
use clean_architecture_domain::use_cases::scope::response::{
    DeleteScopeUseCaseResponse, GetScopesUseCaseResponse, ScopeUseCaseResponse,
};
use clean_architecture_domain::use_cases::scope::ScopeUseCase;

fn init_use_case() -> ScopeUseCase<TestScopeRepository> {
    let scope_repository = TestScopeRepository {};
    let scope_service = ScopeService::new(scope_repository);
    ScopeUseCase::new(scope_service)
}

#[tokio::test]
async fn test_create_scope_use_case() {
    let use_case = init_use_case();
    let request = CreateScopeUseCaseRequest {
        id: SCOPE_ID.to_string(),
    };

    assert!(use_case.create(request).await.is_ok());
}

#[tokio::test]
async fn test_get_scopes_use_case() {
    let use_case = init_use_case();
    let scopes = GetScopesUseCaseResponse {
        scopes: vec![ScopeUseCaseResponse {
            id: SCOPE_ID.to_string(),
            created_at: DateTime::parse_from_rfc3339(DATE).unwrap().with_timezone(&Utc),
        }],
    };

    assert_eq!(use_case.get_scopes().await, Ok(scopes));
}

#[tokio::test]
async fn test_delete_scope_use_case_success() {
    let use_case = init_use_case();
    let request = DeleteScopeUseCaseRequest {
        id: SCOPE_ID.to_string(),
    };

    assert_eq!(
        use_case.delete(request).await,
        Ok(DeleteScopeUseCaseResponse { deleted: 1 })
    );
}

#[tokio::test]
async fn test_delete_scope_use_case_not_found() {
    let use_case = init_use_case();
    let request = DeleteScopeUseCaseRequest { id: "test".to_string() };

    assert!(use_case.delete(request).await.is_err());
}
