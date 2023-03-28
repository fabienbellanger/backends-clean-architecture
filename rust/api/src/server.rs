//! Server module

use crate::config::Config;
use crate::layers::{
    states::{SharedState, State},
    MakeRequestUuid,
};
use crate::{handlers, layers, logger, routes};
use axum::error_handling::HandleErrorLayer;
use axum::{Extension, Router};
use clean_architecture_database::init_mysql_pool;
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
use std::net::{AddrParseError, SocketAddr};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::ServiceBuilderExt;

/// Starts API server
pub async fn start_server() -> ApiResult<()> {
    // Load configuration
    let settings = Config::from_env()?;

    // Get router
    let app = get_app(&settings).await?;

    // Start server
    let addr = format!("{}:{}", settings.server_url, settings.server_port);
    info!("Starting server on {}...", &addr);

    let server = axum::Server::bind(
        &addr
            .parse()
            .map_err(|err: AddrParseError| api_error!(ApiErrorCode::InternalError, err))?,
    )
    .serve(app.into_make_service_with_connect_info::<SocketAddr>());

    server
        .await
        .map_err(|err| api_error!(ApiErrorCode::InternalError, err))
}

/// Initialize router
async fn get_app(settings: &Config) -> ApiResult<Router> {
    // Tracing
    logger::init(
        &settings.environment,
        &settings.logs_path,
        &settings.logs_file,
    )?;

    // MySQL database
    let pool = init_mysql_pool().await?;

    // CORS
    let cors = layers::cors(settings);

    // Layers
    let layers = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .layer(layers::logger::LoggerLayer)
        .layer(HandleErrorLayer::new(handlers::timeout_error))
        .timeout(Duration::from_secs(settings.request_timeout))
        .propagate_x_request_id();

    // Global state
    let global_state = SharedState::new(State::init(settings));

    let mut app = Router::new()
        .nest("/api/v1", routes::api(global_state.clone()))
        .layer(cors);
    app = app.layer(Extension(pool)).layer(layers);

    let app = app.with_state(global_state);

    Ok(app)
}
