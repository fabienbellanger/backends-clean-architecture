//! Routes list

use crate::layers::states::SharedState;
use crate::{handlers, layers};
use axum::routing::{get, post};
use axum::Router;

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
        // .route("/", post(handlers::users::create))
        .route("/", get(handlers::users::get_users))
        .route("/:id", get(handlers::users::get_user))
}
