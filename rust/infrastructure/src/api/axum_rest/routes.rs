//! Routes list

use super::{
    handlers,
    layers::{auth::JwtLayer, basic_auth::BasicAuthLayer, states::SharedState},
};
use crate::auth;
use crate::config::Config;
use axum::routing::{delete, get, patch, post};
use axum::Router;
use clean_architecture_domain::entities::scope::{SCOPE_ADMIN, SCOPE_USERS};
use metrics_exporter_prometheus::PrometheusHandle;
use std::future::ready;

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

/// Add Prometheus route
pub fn prometheus(settings: &Config, handle: PrometheusHandle) -> Router<SharedState> {
    Router::new().route(
        "/",
        get(move || ready(handle.render())).layer(BasicAuthLayer::new(
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
        .route("/forgotten-password/:email", post(handlers::users::forgotten_password))
        .route("/update-password/:token", patch(handlers::users::update_password))
        .route("/refresh-token/:token", post(handlers::users::refresh_token))
        // Private routes
        .nest("/", api_protected(state))
}

/// Protected API routes
fn api_protected(state: SharedState) -> Router<SharedState> {
    Router::new()
        .nest("/scopes", api_scopes().layer(auth!(state.clone(), SCOPE_ADMIN)))
        .nest("/users", api_users().layer(auth!(state, SCOPE_USERS)))
}

/// Users API routes
fn api_users() -> Router<SharedState> {
    Router::new()
        .route("/", post(handlers::users::create_user))
        .route("/", get(handlers::users::get_users))
        .route("/:id", get(handlers::users::get_user))
        .route("/:id", delete(handlers::users::delete_user))
        .route("/:id/scopes", get(handlers::users::get_scopes))
}

/// Scopes API routes
fn api_scopes() -> Router<SharedState> {
    Router::new()
        .route("/", post(handlers::scopes::create))
        .route("/", get(handlers::scopes::get_all))
        .route("/:id", delete(handlers::scopes::delete))
}
