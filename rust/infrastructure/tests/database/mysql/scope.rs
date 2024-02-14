use crate::helpers::mysql::TestMySQL;
use clean_architecture_domain::repositories::scope::ScopeRepository;
use clean_architecture_domain::requests::scope::{CreateRequest, DeleteRequest};
use clean_architecture_infrastructure::database::mysql::repositories::scope::ScopeMysqlRepository;

#[tokio::test]
async fn test_create_scope() {
    let db = TestMySQL::new().await;
    let repository = ScopeMysqlRepository::new(db.database());

    // Create a new scope
    let request = CreateRequest {
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
    let request = CreateRequest {
        id: "scope1".to_string(),
    };
    repository.create(request.clone()).await.unwrap();
    let request = CreateRequest {
        id: "scope2".to_string(),
    };
    repository.create(request.clone()).await.unwrap();

    // Get all scopes
    let scopes = repository.get_scopes().await;
    assert!(scopes.is_ok());

    if let Ok(scopes) = scopes {
        assert_eq!(scopes.len(), 2);
    }
}

#[tokio::test]
async fn test_delete_scope() {
    let db = TestMySQL::new().await;
    let repository = ScopeMysqlRepository::new(db.database());

    // Create a new scope
    let request = CreateRequest {
        id: "scope".to_string(),
    };
    repository.create(request).await.unwrap();

    // Delete the scope
    let result = repository
        .delete(DeleteRequest {
            id: "scope".to_string(),
        })
        .await;
    assert!(result.is_ok());

    // Get all scopes
    let scopes = repository.get_scopes().await;
    assert!(scopes.is_ok());

    if let Ok(scopes) = scopes {
        assert_eq!(scopes.len(), 0);
    }
}
