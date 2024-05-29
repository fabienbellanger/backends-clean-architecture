//! Error entity

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

/// Custom Result typefor `CliError`
pub type CliResult<T> = Result<T, CliError>;

/// Custom CLI Error
#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum CliError {
    #[error("Panic: {0}")]
    Panic(String),

    #[error("Config error: {0}")]
    ConfigError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("CLI error: {0}")]
    Error(String),

    #[error("Server error: {0}")]
    ServerError(String),
}

impl From<ApiError> for CliError {
    fn from(err: ApiError) -> Self {
        Self::ServerError(err.to_string())
    }
}

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
    PayloadTooLarge,
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

    #[error("Payload Too Large")]
    PayloadTooLarge,
}

// Axum errors
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
            ApiError::PayloadTooLarge { .. } => StatusCode::PAYLOAD_TOO_LARGE,
        };

        let body = Json(json!(ApiErrorMessage {
            code: status.as_u16(),
            message: self.to_string(),
        }));

        (status, body).into_response()
    }
}

// SQLx errors
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
            ApiErrorCode::PayloadTooLarge => ApiError::PayloadTooLarge,
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
            ApiErrorCode::PayloadTooLarge => ApiError::PayloadTooLarge,
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
            ApiErrorCode::PayloadTooLarge => ApiError::PayloadTooLarge,
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

#[cfg(test)]
mod test {
    use super::{ApiError, CliError};

    #[test]
    fn test_from_api_error_to_cli_error() {
        let api_error = ApiError::Unauthorized;
        let expected = CliError::ServerError("Unauthorized".to_owned());
        assert_eq!(CliError::from(api_error), expected);

        let api_error = ApiError::NotFound {
            message: "resource not found".to_owned(),
        };
        let expected = CliError::ServerError("resource not found".to_owned());
        assert_eq!(CliError::from(api_error), expected);
    }
}
