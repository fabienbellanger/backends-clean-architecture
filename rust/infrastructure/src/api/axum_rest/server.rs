//! Server

use super::layers::{
    prometheus::PrometheusMetric,
    states::{SharedState, State},
    MakeRequestUuid,
};
use super::usecases::AppUseCases;
use super::{handlers, layers, logger, routes};
use crate::config::Config;
use crate::database::{mysql::Db, GenericDb};
use crate::email::{Email, EmailConfig};
use axum::{error_handling::HandleErrorLayer, middleware, Extension, Router};
use clean_architecture_shared::{
    api_error,
    error::{ApiError, ApiErrorCode, ApiResult},
};
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::signal;
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
    let listener = TcpListener::bind(&addr).await.unwrap();
    info!("Starting server on {}...", &addr);

    let server = axum::serve(listener, app);

    // Graceful shutdown only in production environment
    // TODO: https://github.com/tokio-rs/axum/blob/main/examples/graceful-shutdown/src/main.rs
    server.await.map_err(|err| api_error!(ApiErrorCode::InternalError, err))
    // if settings.environment != "production" {
    //     server.await.map_err(|err| api_error!(ApiErrorCode::InternalError, err))
    // } else {
    //     server
    //         .with_graceful_shutdown(shutdown_signal())
    //         .await
    //         .map_err(|err| api_error!(ApiErrorCode::InternalError, err))
    // }
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
    let global_state = SharedState::new(State::init(settings)?);

    // Routing - API
    let mut app = Router::new()
        .nest("/api/v1", routes::api(global_state.clone()))
        .layer(cors);

    // Routing - Web
    app = app.nest("/", routes::web(settings));

    // Prometheus metrics
    if settings.prometheus_metrics_enabled {
        app = app
            .nest(
                "/metrics",
                routes::prometheus(settings, PrometheusMetric::get_handle()?),
            )
            .route_layer(middleware::from_fn(PrometheusMetric::get_layer));
    }

    // Database
    let db = Db::new().await?;

    // Email
    let email = Email::new(EmailConfig::from(settings.clone()));

    app = app
        .fallback_service(ServeDir::new("assets").append_index_html_on_directories(true)) // FIXME: static_file_error not work this Axum 0.6.9!
        .layer(middleware::from_fn(layers::override_http_errors))
        .layer(layers)
        .layer(Extension(AppUseCases::new(db, email).await?));

    // State
    let app = app.with_state(global_state);

    Ok(app)
}

async fn _shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    info!("signal received, starting graceful shutdown");
}
