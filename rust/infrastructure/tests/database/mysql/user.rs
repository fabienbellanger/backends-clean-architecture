use crate::helpers::mysql::TestMySQL;
use clean_architecture_domain::repositories::scope::ScopeRepository;
use clean_architecture_domain::requests::scope::CreateRequest;
use clean_architecture_domain::requests::user::UserScopeRequest;
use clean_architecture_domain::{
    value_objects::email::Email,
    {
        repositories::user::UserRepository,
        requests::user::{CreateUserRequest, UserIdRequest},
    },
};
use clean_architecture_infrastructure::database::mysql::repositories::scope::ScopeMysqlRepository;
use clean_architecture_infrastructure::database::mysql::repositories::user::UserMysqlRepository;

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
        assert_eq!(user.email, Email::new(&request.email).unwrap());
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
    let request = UserIdRequest { id: user.id };
    let result = repository.get_user_by_id(request.clone()).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_add_scope() {
    let db = TestMySQL::new().await;
    let repository = UserMysqlRepository::new(db.database());

    // Create a new user
    let request = CreateUserRequest {
        lastname: "Doe".to_string(),
        firstname: "John".to_string(),
        email: "john.doe@test.com".to_string(),
        password: "00000000".to_string(),
    };
    let user_id = repository.create_user(request.clone()).await.unwrap().id;

    // Create a new scope
    let scope_id = "test:read".to_string();
    let scope_repository = ScopeMysqlRepository::new(db.database());
    scope_repository
        .create(CreateRequest { id: scope_id.clone() })
        .await
        .unwrap();

    // Add scope to user
    let result = repository.add_scope(UserScopeRequest { user_id, scope_id }).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_remove_scope() {
    let db = TestMySQL::new().await;
    let repository = UserMysqlRepository::new(db.database());

    // Create a new user
    let request = CreateUserRequest {
        lastname: "Doe".to_string(),
        firstname: "John".to_string(),
        email: "john.doe@test.com".to_string(),
        password: "00000000".to_string(),
    };
    let user_id = repository.create_user(request.clone()).await.unwrap().id;

    // Create a new scope
    let scope_id = "test:read".to_string();
    let scope_repository = ScopeMysqlRepository::new(db.database());
    scope_repository
        .create(CreateRequest { id: scope_id.clone() })
        .await
        .unwrap();

    // Add scope to user
    repository
        .add_scope(UserScopeRequest {
            user_id,
            scope_id: scope_id.clone(),
        })
        .await
        .unwrap();

    // Remove scope from user
    let result = repository.remove_scope(UserScopeRequest { user_id, scope_id }).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_scopes() {
    let db = TestMySQL::new().await;
    let repository = UserMysqlRepository::new(db.database());

    // Create a new user
    let request = CreateUserRequest {
        lastname: "Doe".to_string(),
        firstname: "John".to_string(),
        email: "john.doe@test.com".to_string(),
        password: "00000000".to_string(),
    };
    let user_id = repository.create_user(request.clone()).await.unwrap().id;

    // Create a new scope
    let scope_id = "test:read".to_string();
    let scope_repository = ScopeMysqlRepository::new(db.database());
    scope_repository
        .create(CreateRequest { id: scope_id.clone() })
        .await
        .unwrap();

    // Add scope to user
    repository
        .add_scope(UserScopeRequest {
            user_id,
            scope_id: scope_id.clone(),
        })
        .await
        .unwrap();

    // Gets scopes
    let result = repository.get_scopes(user_id).await;
    assert!(result.is_ok());

    if let Ok(scopes) = result {
        assert_eq!(scopes.len(), 1);
        assert_eq!(scopes[0].id, scope_id);
    }
}
