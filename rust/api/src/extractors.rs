//! Custom Axum extractors

use axum::http::{header::HeaderValue, request::Parts};
use axum::{async_trait, extract::FromRequestParts};

/// Request ID extractor from HTTP headers
pub struct ExtractRequestId(pub HeaderValue);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractRequestId
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.headers.get("x-request-id") {
            Some(id) => Ok(ExtractRequestId(id.clone())),
            _ => Ok(ExtractRequestId(HeaderValue::from_static(""))),
        }
    }
}
