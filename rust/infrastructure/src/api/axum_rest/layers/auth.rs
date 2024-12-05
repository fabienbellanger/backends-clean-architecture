//! JWT layer

use super::body_from_parts;
use super::states::SharedState;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use clean_architecture_shared::auth::{Claims, ClaimsExtractor};
use futures::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};

#[derive(Clone)]
pub struct JwtLayer<'a> {
    pub state: SharedState,
    pub scopes: Vec<&'a str>,
}

impl<'a, S> Layer<S> for JwtLayer<'a> {
    type Service = JwtMiddleware<'a, S>;

    fn layer(&self, inner: S) -> Self::Service {
        JwtMiddleware {
            inner,
            state: self.state.clone(),
            scopes: self.scopes.clone(),
        }
    }
}

#[derive(Clone)]
pub struct JwtMiddleware<'a, S> {
    inner: S,
    state: SharedState,
    scopes: Vec<&'a str>,
}

impl<S> Service<Request<Body>> for JwtMiddleware<'_, S>
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
        let is_authorized = match Claims::from_request(request.headers(), &self.state.jwt) {
            Some(Ok(claims)) => {
                let user_scopes = claims.scopes.iter().map(|s| s.value()).collect::<Vec<_>>();

                self.scopes.iter().all(|s| user_scopes.contains(s))
            }
            _ => false,
        };

        let future = self.inner.call(request);
        Box::pin(async move {
            let mut response = Response::default();

            response = match is_authorized {
                true => future.await?,
                false => {
                    let (mut parts, _body) = response.into_parts();
                    let msg = body_from_parts(&mut parts, StatusCode::UNAUTHORIZED, "Unauthorized", None);
                    Response::from_parts(parts, Body::from(msg))
                }
            };

            Ok(response)
        })
    }
}

/// Add jwt layer to the service with application state and scopes
///
/// # Examples
///
/// ```no_run,ignore
/// // Without scope
/// Router::new()
///     .route("public", get(public))
///     .route("/admin", post(login)).layer(auth!(state)))
///
/// // With scopes
/// Router::new()
///     .route("/admin", post(register)).layer(auth!(state, "users", "posts")))
/// ```
#[macro_export]
macro_rules! auth {
    ($state:expr) => {
        JwtLayer { state: $state, scopes: vec![] }
    };

    ($state:expr, $($scope:expr),*) => {
        JwtLayer { state: $state, scopes: vec![$(($scope)),*] }
    };
}
