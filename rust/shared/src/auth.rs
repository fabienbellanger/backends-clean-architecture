//! Authentification module

use crate::api_error;
use crate::error::{ApiError, ApiErrorCode, ApiResult};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    pub user_id: String,
}

pub trait ClaimsExtractor<H> {
    /// Extract claims from request headers
    fn from_request(headers: &H, decoding_key: &DecodingKey) -> Option<ApiResult<Claims>>;
}

pub struct Jwt {
    /// The algorithm supported for signing/verifying JWT
    algorithm: Algorithm,

    /// Token lifetime
    lifetime: i64,

    /// Encoding key
    encoding_key: Option<EncodingKey>,

    /// Decoding key
    decoding_key: Option<DecodingKey>,
}

impl Debug for Jwt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Jwt => algo: {:?}, lifetime: {:?}",
            self.algorithm, self.lifetime
        )
    }
}

impl Jwt {
    /// Create a new `Jwt`
    pub fn new(
        algorithm: Algorithm,
        lifetime: i64,
        encoding_key: Option<EncodingKey>,
        decoding_key: Option<DecodingKey>,
    ) -> Self {
        Self {
            algorithm,
            lifetime,
            encoding_key,
            decoding_key,
        }
    }

    /// Generate JWT
    pub fn generate(&self, user_id: String) -> ApiResult<(String, i64)> {
        let header = jsonwebtoken::Header::new(self.algorithm);
        let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
        let expired_at = now + (self.lifetime * 3600);

        let payload = Claims {
            sub: user_id.clone(),
            exp: expired_at,
            iat: now,
            nbf: now,
            user_id,
        };

        match self.encoding_key.clone() {
            Some(encoding_key) => {
                let token = encode(&header, &payload, &encoding_key).map_err(|err| {
                    api_error!(
                        ApiErrorCode::InternalError,
                        "error during JWT encoding",
                        format!("error during JWT encoding: {err}")
                    )
                })?;

                Ok((token, expired_at))
            }
            _ => Err(api_error!(
                ApiErrorCode::InternalError,
                "error during JWT encoding",
                format!("error during JWT encoding: no encoding key")
            )),
        }
    }

    /// Parse JWT
    pub fn parse(&self, token: &str) -> ApiResult<Claims> {
        let validation = Validation::new(self.algorithm);
        match self.decoding_key.clone() {
            Some(decoding_key) => {
                let token = decode::<Claims>(token, &decoding_key, &validation).map_err(|err| {
                    api_error!(
                        ApiErrorCode::InternalError,
                        "error during JWT decoding",
                        format!("error during JWT decoding: {err}")
                    )
                })?;

                Ok(token.claims)
            }
            _ => Err(api_error!(
                ApiErrorCode::InternalError,
                "error during JWT decoding",
                format!("error during JWT decoding: no decoding key")
            )),
        }
    }
}
