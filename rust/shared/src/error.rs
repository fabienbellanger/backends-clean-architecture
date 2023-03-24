//! Error entity module

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

/// Custom Result type for `ApiError`
pub type ApiResult<T> = Result<T, ApiError>;

/// Represents the custom error message
#[derive(Deserialize, Serialize)]
pub struct ApiErrorMessage {
    pub code: u16,
    pub message: String,
}

#[derive(Debug)]
pub enum ApiErrorCode {
    InternalError,
    BadRequest,
    NotFound,
    UnprocessableEntity,
    Timeout,
    Unauthorized,
    TooManyRequests,
    MethodNotAllowed,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ApiError {
    #[error("{message}")]
    InternalError { message: String },

    #[error("{message}")]
    BadRequest { message: String },

    #[error("{message}")]
    NotFound { message: String },

    #[error("{message}")]
    UnprocessableEntity { message: String },

    #[error("Request Timeout")]
    Timeout,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Too Many Requests")]
    TooManyRequests,

    #[error("Method Not Allowed")]
    MethodNotAllowed,
}

// Axum errors
// ------------
impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let status = match self {
            ApiError::InternalError { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            ApiError::NotFound { .. } => StatusCode::NOT_FOUND,
            ApiError::Unauthorized { .. } => StatusCode::UNAUTHORIZED,
            ApiError::BadRequest { .. } => StatusCode::BAD_REQUEST,
            ApiError::Timeout { .. } => StatusCode::REQUEST_TIMEOUT,
            ApiError::TooManyRequests { .. } => StatusCode::TOO_MANY_REQUESTS,
            ApiError::MethodNotAllowed { .. } => StatusCode::METHOD_NOT_ALLOWED,
            ApiError::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
        };

        let body = Json(json!(ApiErrorMessage {
            code: status.as_u16(),
            message: self.to_string(),
        }));

        (status, body).into_response()
    }
}

// SQLx errors
// -----------
impl From<sqlx::Error> for ApiError {
    fn from(error: sqlx::Error) -> Self {
        error!("Database error: {error:?}");

        Self::InternalError {
            message: "Database Error".to_owned(),
        }
    }
}

/// Create an [`ApiError`] and generate a log if HTTP Code is 500.
///
/// ```rust
/// use clean_architecture_shared::error::{ApiError, ApiErrorCode, ApiResult};
/// use clean_architecture_shared::api_error;
///
/// #[macro_use]
/// extern crate tracing;
///
/// fn main() -> ApiResult<()> {
///     assert_eq!(ApiError::Timeout, api_error!(ApiErrorCode::Timeout));
///     assert_eq!(
///         ApiError::InternalError{ message: "Internal Server Error".to_owned()},
///         api_error!(ApiErrorCode::InternalError)
///     );
///
///     assert_eq!(ApiError::Timeout, api_error!(ApiErrorCode::Timeout, "Timeout"));
///     assert_eq!(
///         ApiError::InternalError{ message: "My error".to_owned()},
///         api_error!(ApiErrorCode::InternalError, "My error")
///     );
///
///     assert_eq!(
///         ApiError::InternalError{ message: "My error".to_owned()},
///         api_error!(ApiErrorCode::InternalError, "My error", "Details of my error")
///     );
///     
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! api_error {
    ( $error:expr ) => {
        match $error {
            ApiErrorCode::Timeout => ApiError::Timeout,
            ApiErrorCode::Unauthorized => ApiError::Unauthorized,
            ApiErrorCode::TooManyRequests => ApiError::TooManyRequests,
            ApiErrorCode::MethodNotAllowed => ApiError::MethodNotAllowed,
            ApiErrorCode::InternalError => ApiError::InternalError {
                message: String::from("Internal Server Error"),
            },
            ApiErrorCode::BadRequest => ApiError::BadRequest {
                message: String::from("Bad Request"),
            },
            ApiErrorCode::NotFound => ApiError::NotFound {
                message: String::from("Not Found"),
            },
            ApiErrorCode::UnprocessableEntity => ApiError::UnprocessableEntity {
                message: String::from("Unprocessable Entity"),
            },
        }
    };

    ( $error:expr, $message:expr ) => {
        match $error {
            ApiErrorCode::Timeout => ApiError::Timeout,
            ApiErrorCode::Unauthorized => ApiError::Unauthorized,
            ApiErrorCode::TooManyRequests => ApiError::TooManyRequests,
            ApiErrorCode::MethodNotAllowed => ApiError::MethodNotAllowed,
            ApiErrorCode::InternalError => {
                error!("{}", $message);
                ApiError::InternalError {
                    message: $message.to_string(),
                }
            }
            ApiErrorCode::BadRequest => ApiError::BadRequest {
                message: $message.to_string(),
            },
            ApiErrorCode::NotFound => ApiError::NotFound {
                message: $message.to_string(),
            },
            ApiErrorCode::UnprocessableEntity => ApiError::UnprocessableEntity {
                message: $message.to_string(),
            },
        }
    };

    ( $error:expr, $message:expr, $details:expr ) => {
        match $error {
            ApiErrorCode::Timeout => ApiError::Timeout,
            ApiErrorCode::Unauthorized => ApiError::Unauthorized,
            ApiErrorCode::TooManyRequests => ApiError::TooManyRequests,
            ApiErrorCode::MethodNotAllowed => ApiError::MethodNotAllowed,
            ApiErrorCode::InternalError => {
                error!("{}", $details);
                ApiError::InternalError {
                    message: $message.to_string(),
                }
            }
            ApiErrorCode::BadRequest => ApiError::BadRequest {
                message: $message.to_string(),
            },
            ApiErrorCode::NotFound => ApiError::NotFound {
                message: $message.to_string(),
            },
            ApiErrorCode::UnprocessableEntity => ApiError::UnprocessableEntity {
                message: $message.to_string(),
            },
        }
    };
}
