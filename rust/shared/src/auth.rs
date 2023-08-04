//! Authentification module

use crate::api_error;
use crate::error::{ApiError, ApiErrorCode, ApiResult};
use axum::http::{header, HeaderMap};
use chrono::Utc;
use jsonwebtoken::errors::ErrorKind::ExpiredSignature;
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
    fn from_request(headers: &H, jwt: &Jwt) -> Option<ApiResult<Claims>>;
}

impl ClaimsExtractor<HeaderMap> for Claims {
    fn from_request(headers: &HeaderMap, jwt: &Jwt) -> Option<ApiResult<Claims>> {
        headers
            .get(header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| {
                let words = h.split("Bearer").collect::<Vec<&str>>();
                words.get(1).map(|w| w.trim())
            })
            .map(|token| jwt.parse(token))
    }
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

impl Default for Jwt {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::HS512,
            lifetime: 24, // 24h
            encoding_key: None,
            decoding_key: None,
        }
    }
}

impl Debug for Jwt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Jwt => algo: {:?}, lifetime: {:?}", self.algorithm, self.lifetime)
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

    /// Update lifetime
    pub fn set_lifetime(&mut self, hours: i64) {
        self.lifetime = hours;
    }

    /// Update encoding key
    pub fn set_encoding_key(&mut self, secret: &str) {
        self.encoding_key = Some(EncodingKey::from_secret(secret.as_bytes()));
    }

    /// Update decoding key
    pub fn set_decoding_key(&mut self, secret: &str) {
        self.decoding_key = Some(DecodingKey::from_secret(secret.as_bytes()));
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
                let token = decode::<Claims>(token, &decoding_key, &validation).map_err(|err| match err.kind() {
                    ExpiredSignature => api_error!(
                        ApiErrorCode::InternalError,
                        "error during JWT decoding",
                        format!("error during JWT decoding: {err}")
                    ),
                    _ => api_error!(ApiErrorCode::InternalError),
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
