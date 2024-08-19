pub mod scope;
pub mod user;

use self::user::create_and_authenticate;
use crate::helpers::api::axum_rest::scope::create_scopes;
use crate::helpers::mysql::TestMySQL;
use axum::body::Body;
use axum::Router;
use axum::{http::StatusCode, Extension};
use clean_architecture_infrastructure::api::axum_rest::layers::REQUEST_ID_HEADER;
use clean_architecture_infrastructure::api::axum_rest::{
    layers::{
        self,
        states::{ConfigState, SharedState, State},
        MakeRequestUuid,
    },
    logger, routes,
    use_cases::AppUseCases,
};
use clean_architecture_infrastructure::config::Config;
use clean_architecture_infrastructure::email::{Email, EmailConfig};
use clean_architecture_shared::{auth::Jwt, error::ApiErrorMessage};
use http_body_util::BodyExt;
use hyper::Request;
use serde_json::Value;
use std::collections::HashMap;
use tower::{ServiceBuilder, ServiceExt};
use tower_http::request_id::SetRequestIdLayer;

// HTTP response for test
#[derive(Debug)]
pub struct TestResponse {
    pub status_code: StatusCode,
    pub headers: HashMap<String, String>,
    pub body: Value,
}

impl TestResponse {
    /// Create a new `TestResponse`
    #[allow(dead_code, unused_variables)]
    pub async fn new(app: &TestApp, url: &str, method: &str, body: Option<String>, token: Option<&str>) -> Self {
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
        let body = response
            .into_body()
            .collect()
            .await
            .expect("failed to convert body into bytes")
            .to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap_or(Value::Null);

        TestResponse {
            status_code,
            body,
            headers: HashMap::new(),
        }
    }
}

impl TryInto<ApiErrorMessage> for TestResponse {
    type Error = serde_json::Error;

    fn try_into(self) -> Result<ApiErrorMessage, Self::Error> {
        serde_json::from_str(&self.body.to_string())
    }
}

pub struct TestApp {
    pub router: Router,
    pub database: TestMySQL,
}

impl TestApp {
    /// Create a new user and return a tuple with login response and token
    pub async fn make_authentication(&self) -> (TestResponse, String) {
        create_scopes(&self).await;
        create_and_authenticate(self).await
    }
}

pub struct TestAppBuilder {
    router: Router,
    database: TestMySQL,
}

impl TestAppBuilder {
    pub async fn new() -> Self {
        let state = Self::get_state();
        let settings = Config::default();
        let database = TestMySQL::new().await;
        let email = Email::new(EmailConfig::default());

        let mut router = Router::new().nest("/api/v1", routes::api(state.clone()));
        router = router.nest("/", routes::web(&settings));
        router = router.layer(Extension(AppUseCases::new(database.database(), email).await.unwrap()));

        let router = router.with_state(state);

        Self { router, database }
    }

    fn get_state() -> SharedState {
        let jwt_secret_key = "mySecretJwtKey";
        let jwt_access_lifetime = 24;
        let jwt_refresh_lifetime = 24;

        let state = State {
            config: ConfigState {
                request_body_max_size: 1_024 * 1_024,
                forgotten_password_expiration_duration: 24,
            },
            jwt: Jwt::new(
                "HS512",
                jwt_access_lifetime,
                jwt_refresh_lifetime,
                Some(jwt_secret_key),
                None,
                None,
            )
                .unwrap(),
        };

        SharedState::new(state)
    }

    #[allow(unused)]
    pub fn with_logger(self) -> Self {
        logger::init("test", "", "").unwrap();
        let layers = ServiceBuilder::new()
            .layer(SetRequestIdLayer::new(REQUEST_ID_HEADER.clone(), MakeRequestUuid))
            .layer(layers::logger::LoggerLayer)
            .into_inner();

        Self {
            router: self.router.layer(layers),
            database: self.database,
        }
    }

    pub fn build(self) -> TestApp {
        TestApp {
            router: self.router,
            database: self.database,
        }
    }
}
