use crate::helpers::mysql::TestMySQL;
use clean_architecture_domain::repositories::scope::request::{
    CreateScopeRepositoryRequest, DeleteScopeRepositoryRequest,
};
use clean_architecture_domain::repositories::scope::ScopeRepository;
use clean_architecture_infrastructure::database::mysql::repositories::scope::ScopeMysqlRepository;

#[tokio::test]
async fn test_create_scope() {
    let db = TestMySQL::new().await;
    let repository = ScopeMysqlRepository::new(db.database());

    // Create a new scope
    let request = CreateScopeRepositoryRequest {
        id: "scope1".to_string(),
    };
    let result = repository.create(request.clone()).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_scopes() {
    let db = TestMySQL::new().await;
    let repository = ScopeMysqlRepository::new(db.database());

    // Create new scopes
    let request = CreateScopeRepositoryRequest {
        id: "scope1".to_string(),
    };
    repository.create(request.clone()).await.unwrap();
    let request = CreateScopeRepositoryRequest {
        id: "scope2".to_string(),
    };
    repository.create(request.clone()).await.unwrap();

    // Get all scopes
    let response = repository.get_scopes().await;
    assert!(response.is_ok());

    if let Ok(response) = response {
        assert_eq!(response.scopes.len(), 2);
    }
}

#[tokio::test]
async fn test_delete_scope() {
    let db = TestMySQL::new().await;
    let repository = ScopeMysqlRepository::new(db.database());

    // Create a new scope
    let request = CreateScopeRepositoryRequest {
        id: "scope".to_string(),
    };
    repository.create(request).await.unwrap();

    // Delete the scope
    let result = repository
        .delete(DeleteScopeRepositoryRequest {
            id: "scope".to_string(),
        })
        .await;
    assert!(result.is_ok());

    // Get all scopes
    let response = repository.get_scopes().await;
    assert!(response.is_ok());

    if let Ok(response) = response {
        assert_eq!(response.scopes.len(), 0);
    }
}
