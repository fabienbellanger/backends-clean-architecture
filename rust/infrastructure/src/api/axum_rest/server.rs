//! Server module

use super::layers::{
    basic_auth::BasicAuthLayer,
    prometheus::PrometheusMetric,
    states::{SharedState, State},
    MakeRequestUuid,
};
use super::usecases::AppUseCases;
use super::{handlers, layers, logger, routes};
use crate::config::Config;
use crate::database::mysql::Db;
use crate::database::GenericDb;
use crate::email::{Email, EmailConfig};
use axum::{error_handling::HandleErrorLayer, routing::get};
use axum::{middleware, Extension, Router};
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
use std::{future::ready, time::Duration};
use std::{
    net::{AddrParseError, SocketAddr},
    sync::Arc,
};
use tower::ServiceBuilder;
use tower_http::{services::ServeDir, ServiceBuilderExt};

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

    server.await.map_err(|err| api_error!(ApiErrorCode::InternalError, err))
}

/// Initialize router
async fn get_app(settings: &Config) -> ApiResult<Router> {
    // Tracing
    logger::init(&settings.environment, &settings.logs_path, &settings.logs_file)?;

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

    // Routing - API
    let mut app = Router::new()
        .nest("/api/v1", routes::api(global_state.clone()))
        .layer(cors);

    // Routing - Web
    app = app.nest("/", routes::web(settings));

    // Prometheus metrics
    if settings.prometheus_metrics_enabled {
        let handle = PrometheusMetric::get_handle()?;
        app = app
            .nest(
                "/metrics",
                Router::new().route(
                    "/",
                    get(move || ready(handle.render())).layer(BasicAuthLayer::new(
                        &settings.basic_auth_username,
                        &settings.basic_auth_password,
                    )),
                ),
            )
            .route_layer(middleware::from_fn(PrometheusMetric::get_layer));
    }

    // Database
    let db = Db::new().await?;

    // Email
    let email = Arc::new(Email::new(EmailConfig::from(settings.clone())));

    app = app
        .fallback_service(ServeDir::new("assets").append_index_html_on_directories(true)) // FIXME: static_file_error not work this Axum 0.6.9!
        .layer(middleware::from_fn(layers::override_http_errors))
        .layer(layers)
        .layer(Extension(Arc::new(AppUseCases::new(db, email).await?)));

    // State
    let app = app.with_state(global_state);

    Ok(app)
}
