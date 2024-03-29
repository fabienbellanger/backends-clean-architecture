//! Custom Axum extractors

use axum::extract::path::ErrorKind;
use axum::extract::rejection::PathRejection;
use axum::http::{header, header::HeaderValue, request::Parts, StatusCode};
use axum::{async_trait, extract::FromRequestParts};
use clean_architecture_shared::api_error;
use clean_architecture_shared::error::{ApiError, ApiErrorCode};
use serde::de::DeserializeOwned;

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

/// JWT extractor from HTTP headers
pub struct ExtractJWT(pub Option<String>);

#[async_trait]
impl<S> FromRequestParts<S> for ExtractJWT
where
    S: Send + Sync,
{
    type Rejection = ();

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        match parts.headers.get(header::AUTHORIZATION) {
            Some(token) => {
                let token = token
                    .to_str()
                    .unwrap_or_default()
                    .to_string()
                    .strip_prefix("Bearer ")
                    .map(|s| s.to_string());
                Ok(Self(token))
            }
            _ => Ok(Self(None)),
        }
    }
}

// We define our own `Path` extractor that customizes the error from `axum::extract::Path`
pub struct Path<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for Path<T>
where
    // these trait bounds are copied from `impl FromRequest for axum::extract::path::Path`
    T: DeserializeOwned + Send,
    S: Send + Sync,
{
    type Rejection = (StatusCode, ApiError);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        match axum::extract::Path::<T>::from_request_parts(parts, state).await {
            Ok(value) => Ok(Self(value.0)),
            Err(rejection) => {
                let (status, body) = match rejection {
                    PathRejection::FailedToDeserializePathParams(inner) => {
                        let mut status = StatusCode::BAD_REQUEST;

                        let kind = inner.into_kind();
                        let body = match &kind {
                            ErrorKind::WrongNumberOfParameters { .. } => {
                                api_error!(ApiErrorCode::BadRequest, kind.to_string())
                            }

                            ErrorKind::ParseErrorAtKey { .. } => {
                                api_error!(ApiErrorCode::BadRequest, kind.to_string())
                            }

                            ErrorKind::ParseErrorAtIndex { .. } => {
                                api_error!(ApiErrorCode::BadRequest, kind.to_string())
                            }

                            ErrorKind::ParseError { .. } => {
                                api_error!(ApiErrorCode::BadRequest, kind.to_string())
                            }

                            ErrorKind::InvalidUtf8InPathParam { .. } => {
                                api_error!(ApiErrorCode::BadRequest, kind.to_string())
                            }

                            ErrorKind::UnsupportedType { .. } => {
                                // this error is caused by the programmer using an unsupported type
                                // (such as nested maps) so respond with `500` instead
                                status = StatusCode::INTERNAL_SERVER_ERROR;
                                api_error!(ApiErrorCode::InternalError, kind.to_string())
                            }

                            ErrorKind::Message(msg) => {
                                api_error!(ApiErrorCode::BadRequest, msg.clone())
                            }

                            _ => api_error!(
                                ApiErrorCode::BadRequest,
                                format!("Unhandled deserialization error: {kind}")
                            ),
                        };

                        (status, body)
                    }
                    PathRejection::MissingPathParams(error) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        api_error!(ApiErrorCode::InternalError, error.to_string()),
                    ),
                    _ => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        api_error!(
                            ApiErrorCode::InternalError,
                            format!("Unhandled path rejection: {rejection}")
                        ),
                    ),
                };

                Err((status, body))
            }
        }
    }
}

// We define our own `Query` extractor that customizes the error from `axum::extract::Query`
pub struct Query<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = (StatusCode, ApiError);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let query = parts.uri.query().unwrap_or_default();
        let value = serde_urlencoded::from_str(query).map_err(|err| {
            (
                StatusCode::BAD_REQUEST,
                api_error!(ApiErrorCode::BadRequest, err.to_string()),
            )
        })?;

        Ok(Query(value))
    }
}
