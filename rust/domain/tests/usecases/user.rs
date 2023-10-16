use crate::helpers::password_reset::FORGOTTEN_PASSWORD_TOKEN;
use crate::helpers::user::*;
use crate::helpers::{email::TestEmailService, password_reset::TestPasswordResetRepository};
use chrono::{DateTime, Days, Utc};
use clean_architecture_domain::ports::requests::user::{
    DeleteUserRequest, ForgottenPasswordRequest, UpdateUserPasswordRequest,
};
use clean_architecture_domain::{
    ports::{
        requests::user::{CreateUserRequest, GetUserRequest, LoginRequest},
        responses::user::{GetUserResponse, GetUsersResponse},
        services::user::UserService,
    },
    usecases::user::*,
};
use clean_architecture_shared::error::ApiError;
use clean_architecture_shared::{auth::Jwt, query_parameter::PaginateSort};
use std::cmp::Ordering;
use uuid::Uuid;

fn init_use_case() -> UserUseCase<TestUserRepository, TestPasswordResetRepository, TestEmailService> {
    let user_repository = TestUserRepository {};
    let password_reset_repository = TestPasswordResetRepository {};
    let email_service = TestEmailService {};
    let user_service = UserService::new(user_repository, password_reset_repository);
    UserUseCase::new(user_service, email_service)
}

#[tokio::test]
async fn test_get_user_use_case() {
    let use_case = init_use_case();
    let request = GetUserRequest {
        id: Uuid::parse_str(USER_ID).unwrap(),
    };
    let user = GetUserResponse {
        id: request.id.to_string(),
        lastname: "Doe".to_string(),
        firstname: "John".to_string(),
        email: "john.doe@test.com".to_string(),
        created_at: DATE.to_string(),
        updated_at: DATE.to_string(),
    };

    assert_eq!(use_case.get_user(request).await, Ok(user));
}

#[tokio::test]
async fn test_get_users_use_case() {
    let use_case = init_use_case();
    let users: GetUsersResponse = GetUsersResponse {
        data: vec![GetUserResponse {
            id: Uuid::parse_str(USER_ID).unwrap().to_string(),
            lastname: "Doe".to_string(),
            firstname: "John".to_string(),
            email: "john.doe@test.com".to_string(),
            created_at: DATE.to_string(),
            updated_at: DATE.to_string(),
        }],
        total: TOTAL_USERS,
    };
    let pagination_sort = PaginateSort {
        page: 1,
        limit: 10,
        offset: 0,
        sorts: vec![],
    };

    assert_eq!(use_case.get_users(&pagination_sort).await, Ok(users));
}

#[tokio::test]
async fn test_create_user_use_case() {
    let use_case = init_use_case();
    let request = CreateUserRequest {
        lastname: "Doe".to_string(),
        firstname: "John".to_string(),
        email: "john.doe@test.com".to_string(),
        password: "00000000".to_string(),
    };
    let user = GetUserResponse {
        id: USER_ID.to_string(),
        lastname: "Doe".to_string(),
        firstname: "John".to_string(),
        email: "john.doe@test.com".to_string(),
        created_at: DATE.to_string(),
        updated_at: DATE.to_string(),
    };

    assert_eq!(use_case.create_user(request).await, Ok(user));
}

#[tokio::test]
async fn test_create_user_use_case_with_invalid_request() {
    let use_case = init_use_case();

    // Invalid password
    let request = CreateUserRequest {
        lastname: "Doe".to_string(),
        firstname: "John".to_string(),
        email: "john.doe@test.com".to_string(),
        password: "0000000".to_string(),
    };
    assert!(use_case.create_user(request).await.is_err());

    // Invalid email
    let request = CreateUserRequest {
        lastname: "Doe".to_string(),
        firstname: "John".to_string(),
        email: "john.doe".to_string(),
        password: "00000000".to_string(),
    };
    assert!(use_case.create_user(request).await.is_err());
}

#[tokio::test]
async fn test_login_use_case() {
    let use_case = init_use_case();
    let request = LoginRequest {
        email: USER_EMAIL.to_string(),
        password: "00000000".to_owned(),
    };
    let mut jwt = Jwt::default();
    jwt.set_lifetime(20);
    jwt.set_encoding_key(JWT_SECRET).unwrap();
    jwt.set_decoding_key(JWT_SECRET).unwrap();
    let response = use_case.login(request, &jwt).await.unwrap();
    let expired_at = DateTime::parse_from_rfc3339(&response.expired_at)
        .unwrap()
        .with_timezone(&Utc);
    let tomorrow = Utc::now().checked_add_days(Days::new(1)).unwrap();

    assert_eq!(expired_at.cmp(&tomorrow), Ordering::Less);
    assert!(!response.token.is_empty());
}

#[tokio::test]
async fn test_login_use_case_with_bad_email() {
    let use_case = init_use_case();
    let request = LoginRequest {
        email: "toto@toto.com".to_string(),
        password: "00000000".to_owned(),
    };
    let mut jwt = Jwt::default();
    jwt.set_encoding_key(JWT_SECRET).unwrap();
    jwt.set_decoding_key(JWT_SECRET).unwrap();
    let response = use_case.login(request, &jwt).await;

    assert!(response.is_err());
}

#[tokio::test]
async fn test_delete_user() {
    let use_case = init_use_case();
    let request = DeleteUserRequest {
        id: USER_ID.parse().unwrap(),
    };
    assert_eq!(use_case.delete_user(request).await.unwrap(), 1);

    // Not found user ID
    let request = DeleteUserRequest { id: Uuid::new_v4() };
    assert_eq!(use_case.delete_user(request).await.unwrap(), 0);
}

#[tokio::test]
async fn test_forgotten_password() {
    let use_case = init_use_case();
    let request = ForgottenPasswordRequest {
        email: USER_EMAIL.to_owned(),
        expiration_duration: 1,
    };

    let response = use_case.send_forgotten_password(request).await;
    assert!(response.is_ok());

    // Not found user email
    let request = ForgottenPasswordRequest {
        email: "john.doe@test.com".to_owned(),
        expiration_duration: 1,
    };

    let response = use_case.send_forgotten_password(request).await;
    assert!(response.is_err());

    if let Err(err) = response {
        assert_eq!(
            err,
            ApiError::NotFound {
                message: "no user found".to_owned(),
            }
        );
    }
}

#[tokio::test]
async fn test_update_password() {
    let use_case = init_use_case();
    let request = UpdateUserPasswordRequest {
        token: FORGOTTEN_PASSWORD_TOKEN.to_owned(),
        password: "123456789".to_owned(),
    };
    let response = use_case.update_user_password(request).await;
    assert!(response.is_ok());
}

#[tokio::test]
async fn test_update_password_no_user() {
    let use_case = init_use_case();
    let request = UpdateUserPasswordRequest {
        token: "not_found_token".to_owned(),
        password: "123456789".to_owned(),
    };
    let response = use_case.update_user_password(request).await;
    assert!(response.is_err());

    if let Err(err) = response {
        assert_eq!(
            err,
            ApiError::NotFound {
                message: "no user found".to_owned(),
            }
        );
    }
}
