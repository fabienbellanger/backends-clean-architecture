//! Routes list

use super::config::Config;
use super::layers::basic_auth::BasicAuthLayer;
use super::layers::states::SharedState;
use super::{handlers, layers};
use axum::routing::{delete, get, post};
use axum::Router;

/// Return web routes list
pub fn web(settings: &Config) -> Router<SharedState> {
    Router::new()
        .route("/health-check", get(handlers::web::health_check))
        // API documentation
        .nest(
            "/doc",
            Router::new()
                .route("/api-v1", get(handlers::web::doc_api_v1))
                .layer(BasicAuthLayer::new(
                    &settings.basic_auth_username,
                    &settings.basic_auth_password,
                )),
        )
}

/// Return API routes list
pub fn api(state: SharedState) -> Router<SharedState> {
    Router::new()
        // Public routes
        .route("/login", post(handlers::users::login))
        // Private routes
        .nest("/", api_protected().layer(layers::jwt::JwtLayer { state }))
}

/// Protected API routes
fn api_protected() -> Router<SharedState> {
    Router::new().nest("/users", api_users())
}

/// Users API routes
fn api_users() -> Router<SharedState> {
    Router::new()
        .route("/", post(handlers::users::create_user))
        .route("/", get(handlers::users::get_users))
        .route("/:id", get(handlers::users::get_user))
        .route("/:id", delete(handlers::users::delete_user))
}
