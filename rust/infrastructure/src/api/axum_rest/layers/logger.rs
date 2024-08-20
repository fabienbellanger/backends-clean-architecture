//! Logger layer

use super::header_value_to_str;
use axum::{body::Body, http::Request, response::Response};
use futures::future::BoxFuture;
use std::{
    fmt::Display,
    task::{Context, Poll},
    time::{Duration, Instant},
};
use tower::{Layer, Service};

#[derive(Debug, Default)]
struct LoggerMessage {
    method: String,
    request_id: String,
    host: String,
    uri: String,
    user_agent: String,
    status_code: u16,
    version: String,
    latency: Duration,
}

impl Display for LoggerMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "status_code: {}, method: {}, uri: {}, host: {}, request_id: {}, user_agent: {}, version: {}, latency: {:?}",
               self.status_code,
               self.method,
               self.uri,
               self.host,
               self.request_id,
               self.user_agent,
               self.version,
               self.latency,
        )
    }
}

#[derive(Clone)]
pub struct LoggerLayer;

impl<S> Layer<S> for LoggerLayer {
    type Service = LoggerMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        LoggerMiddleware { inner }
    }
}

#[derive(Clone)]
pub struct LoggerMiddleware<S> {
    inner: S,
}

impl<S> Service<Request<Body>> for LoggerMiddleware<S>
where
    S: Service<Request<Body>, Response = Response> + Send + 'static,

    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // `BoxFuture` is a type alias for `Pin<Box<dyn Future + Send + 'a>>`
    type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request<Body>) -> Self::Future {
        let now = Instant::now();
        let request_headers = request.headers();

        let mut message = LoggerMessage {
            method: request.method().to_string(),
            uri: request.uri().to_string(),
            host: header_value_to_str(request_headers.get("host")).to_string(),
            request_id: header_value_to_str(request_headers.get("x-request-id")).to_string(),
            user_agent: header_value_to_str(request_headers.get("user-agent")).to_string(),
            ..Default::default()
        };

        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;

            message.status_code = response.status().as_u16();
            message.version = format!("{:?}", response.version());
            message.latency = now.elapsed();

            info!(
                status_code = %message.status_code,
                method = %message.method,
                uri = %message.uri,
                host = %message.host,
                request_id = %message.request_id,
                user_agent = %message.user_agent,
                version = %message.version,
                latency = %format!("{:?}", message.latency),
            );

            Ok(response)
        })
    }
}
