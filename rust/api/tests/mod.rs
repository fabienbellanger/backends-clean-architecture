mod api;
mod web;

use axum::{http::StatusCode, Extension, Router};
use clean_architecture_api::config::Config;
use clean_architecture_api::layers::states::*;
use clean_architecture_api::routes;
use clean_architecture_shared::{auth::Jwt, error::ApiErrorMessage, test::mysql::TestMySQL};
use hyper::{Body, Request};
use jsonwebtoken::{DecodingKey, EncodingKey};
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use tower::ServiceExt;

pub struct TestApp {
    pub router: Router,
    pub database: TestMySQL,
}

impl TestApp {
    pub fn database(&self) -> &TestMySQL {
        &self.database
    }
}

pub struct TestAppBuilder {
    router: Router,
    database: TestMySQL,
}

impl TestAppBuilder {
    pub async fn new() -> Self {
        let state = Self::get_state();
        let db = TestMySQL::new().await;
        let settings = Config::default();

        let mut router = Router::new().nest("/api/v1", routes::api(state.clone()));
        router = router.nest("/", routes::web(&settings));
        router = router.layer(Extension(db.database().await));

        let router = router.with_state(state);

        Self {
            router,
            database: db,
        }
    }

    fn get_state() -> SharedState {
        let jwt_secret_key = "mysecretjwtkey";
        let state = State {
            config: ConfigState {
                jwt_encoding_key: EncodingKey::from_secret(jwt_secret_key.as_bytes()),
                jwt_decoding_key: DecodingKey::from_secret(jwt_secret_key.as_bytes()),
                jwt_lifetime: 1025,
            },
            jwt: Jwt::default(),
        };

        SharedState::new(state)
    }

    pub fn build(self) -> TestApp {
        TestApp {
            router: self.router,
            database: self.database,
        }
    }
}

/// HTTP response for test
#[derive(Debug)]
pub struct TestResponse {
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Value,
}

impl TryInto<ApiErrorMessage> for TestResponse {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<ApiErrorMessage, Self::Error> {
        serde_json::from_str(&self.body.to_string())
    }
}

impl TestResponse {
    /// Create a new `TestResponse`
    pub async fn new(
        app: &TestApp,
        url: &str,
        method: &str,
        body: Option<String>,
        token: Option<&str>,
    ) -> Self {
        let mut request = Request::builder()
            .uri(url)
            .method(method)
            .header("Content-Type", "application/json");
        if let Some(token) = token {
            request = request.header("Authorization", format!("Bearer {token}"));
        }

        let request = request.body(match body {
            None => Body::empty(),
            Some(body) => body.into(),
        });

        let response = app.router.clone().oneshot(request.unwrap()).await.unwrap();
        let status_code = response.status();
        let body = hyper::body::to_bytes(response.into_body())
            .await
            .expect("failed to convert body into bytes");
        let body: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);

        TestResponse {
            status_code,
            body,
            headers: HashMap::new(),
        }
    }
}

#[derive(Deserialize)]
pub struct TestPaginateResponse<T> {
    pub data: T,
    pub total: i64,
}
