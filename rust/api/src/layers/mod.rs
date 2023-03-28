//! Layers module

pub mod jwt;
pub mod logger;
pub mod states;

use crate::config::Config;
use axum::headers::HeaderName;
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, ORIGIN};
use axum::http::response::Parts;
use axum::http::{HeaderValue, Method, Request, StatusCode};
use bytes::Bytes;
use clean_architecture_shared::error::ApiErrorMessage;
use hyper::header;
use std::str::from_utf8;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::request_id::{MakeRequestId, RequestId};
use uuid::Uuid;

/// Contruct response body from `Parts`, status code, message and headers
pub fn body_from_parts(
    parts: &mut Parts,
    status_code: StatusCode,
    message: &str,
    headers: Option<Vec<(HeaderName, HeaderValue)>>,
) -> Bytes {
    // Status
    parts.status = status_code;

    // Headers
    parts.headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(mime::APPLICATION_JSON.as_ref()),
    );
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
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
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
