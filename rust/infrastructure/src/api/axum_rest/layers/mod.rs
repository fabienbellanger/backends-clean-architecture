//! Layers

pub(crate) mod auth;
pub(crate) mod basic_auth;
pub mod logger;
pub(crate) mod prometheus;
pub mod states;

use crate::config::Config;
use axum::body::Body;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN};
use axum::http::response::Parts;
use axum::http::{HeaderName, HeaderValue, Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use bytes::Bytes;
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiErrorMessage};
use http_body_util::BodyExt;
use std::str::from_utf8;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::Uuid;

/// Construct response body from `Parts`, status code, message and headers
pub fn body_from_parts(
    parts: &mut Parts,
    status_code: StatusCode,
    message: &str,
    headers: Option<Vec<(HeaderName, HeaderValue)>>,
) -> Bytes {
    // Status
    parts.status = status_code;

    // Headers
    parts
        .headers
        .insert(CONTENT_TYPE, HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()));
    if let Some(headers) = headers {
        for header in headers {
            parts.headers.insert(header.0, header.1);
        }
    }

    // Body
    let msg = serde_json::json!(ApiErrorMessage {
        code: status_code.as_u16(),
        message: String::from(message),
    });

    Bytes::from(msg.to_string())
}

/// Convert `HeaderValue` to `&str`
pub fn header_value_to_str(value: Option<&HeaderValue>) -> &str {
    match value {
        Some(value) => from_utf8(value.as_bytes()).unwrap_or_default(),
        None => "",
    }
}

/// Request ID middleware
#[derive(Clone, Copy)]
pub struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let id = Uuid::new_v4().to_string().parse();
        match id {
            Ok(id) => Some(RequestId::new(id)),
            _ => None,
        }
    }
}

/// CORS layer
pub fn cors(config: &Config) -> CorsLayer {
    let allow_origin = config.cors_allow_origin.clone();

    let layer = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
        .allow_headers([AUTHORIZATION, ACCEPT, ORIGIN, CONTENT_TYPE]);

    if allow_origin == "*" {
        layer.allow_origin(Any)
    } else {
        let origins = allow_origin
            .split(',')
            .filter(|url| *url != "*" && !url.is_empty())
            .filter_map(|url| url.parse().ok())
            .collect::<Vec<HeaderValue>>();

        if origins.is_empty() {
            layer.allow_origin(Any)
        } else {
            layer
                .allow_origin(AllowOrigin::predicate(move |origin: &HeaderValue, _| {
                    origins.contains(origin)
                }))
                .allow_credentials(true)
        }
    }
}

/// Layer which override some HTTP errors by using `AppError`
pub async fn override_http_errors(req: Request<Body>, next: Next) -> impl IntoResponse {
    let response = next.run(req).await;

    // If it is an image, audio or video, we return response
    let headers = response.headers();
    if let Some(content_type) = headers.get("content-type") {
        let content_type = content_type.to_str().unwrap_or_default();
        if content_type.starts_with("image/")
            || content_type.starts_with("audio/")
            || content_type.starts_with("video/")
        {
            return response;
        }
    }

    let (parts, body) = response.into_parts();
    // TODO: Use this => let body_bytes = axum::body::to_bytes(body, 1024).await;
    // match axum::body::to_bytes(body, 2_048_000).await {
    //     Ok(body) => match String::from_utf8(body.to_vec()) {
    //         Ok(body) => match parts.status {
    //             StatusCode::METHOD_NOT_ALLOWED => api_error!(ApiErrorCode::MethodNotAllowed).into_response(),
    //             StatusCode::UNPROCESSABLE_ENTITY => api_error!(ApiErrorCode::UnprocessableEntity, body).into_response(),
    //             _ => Response::from_parts(parts, Body::from(body)),
    //         },
    //         Err(err) => api_error!(ApiErrorCode::InternalError, err.to_string()).into_response(),
    //     },
    //     Err(_) => api_error!(ApiErrorCode::ContentTooLarge).into_response(),
    // }
    // TODO: Body max size hardcoded!
    match body.collect().await {
        Ok(body_bytes) => match String::from_utf8(body_bytes.to_bytes().to_vec()) {
            Ok(body) => match parts.status {
                StatusCode::METHOD_NOT_ALLOWED => api_error!(ApiErrorCode::MethodNotAllowed).into_response(),
                StatusCode::UNPROCESSABLE_ENTITY => api_error!(ApiErrorCode::UnprocessableEntity, body).into_response(),
                _ => Response::from_parts(parts, Body::from(body)),
            },
            Err(err) => api_error!(ApiErrorCode::InternalError, err.to_string()).into_response(),
        },
        Err(err) => api_error!(ApiErrorCode::InternalError, err.to_string()).into_response(),
    }
}
