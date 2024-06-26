//! Authentication

use crate::api_error;
use crate::error::{ApiError, ApiErrorCode, ApiResult};
use axum::http::{header, HeaderMap};
use chrono::Utc;
use jsonwebtoken::errors::ErrorKind::ExpiredSignature;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Validation};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

/// Auth scope
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthScope(String);

impl AuthScope {
    /// Create a new scope
    pub fn new(id: String) -> Self {
        Self(id)
    }

    /// Get the scope
    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
    pub iat: i64,
    pub nbf: i64,
    pub user_id: String,
    pub scopes: Vec<AuthScope>,
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

/// JWT configuration
#[derive(Clone)]
pub struct Jwt {
    /// The algorithm supported for signing/verifying JWT
    algorithm: Algorithm,

    /// Access Token lifetime (in hour)
    access_lifetime: i64,

    /// Refresh Token lifetime (in day)
    pub refresh_lifetime: i64,

    /// Encoding key
    encoding_key: Option<EncodingKey>,

    /// Decoding key
    decoding_key: Option<DecodingKey>,
}

impl Default for Jwt {
    fn default() -> Self {
        Self {
            algorithm: Algorithm::HS512,
            access_lifetime: 15, // 15 minutes
            refresh_lifetime: 7, // 7 days
            encoding_key: None,
            decoding_key: None,
        }
    }
}

impl Debug for Jwt {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Jwt => algo: {:?}, access_lifetime: {:?}, refresh_lifetime: {:?}",
            self.algorithm, self.access_lifetime, self.refresh_lifetime
        )
    }
}

impl Jwt {
    /// Use a secret key instead of a pair of keys
    fn use_secret(&self) -> bool {
        self.algorithm == Algorithm::HS256 || self.algorithm == Algorithm::HS384 || self.algorithm == Algorithm::HS512
    }

    /// Convert `&str` to `Algorithm`
    fn algorithm_from_str(algo: &str) -> ApiResult<Algorithm> {
        Ok(match algo {
            "HS256" => Algorithm::HS256,
            "HS384" => Algorithm::HS384,
            "HS512" => Algorithm::HS512,
            "ES256" => Algorithm::ES256,
            "ES384" => Algorithm::ES384,
            _ => {
                return Err(ApiError::InternalError {
                    message: format!("{algo} is not a valid or supported algorithm"),
                })
            }
        })
    }

    /// Create a new `Jwt`
    pub fn new(
        algorithm: &str,
        access_lifetime: i64,
        refresh_lifetime: i64,
        secret: Option<&str>,
        private_key: Option<&str>,
        public_key: Option<&str>,
    ) -> ApiResult<Self> {
        let mut jwt = Jwt {
            algorithm: Self::algorithm_from_str(algorithm)?,
            access_lifetime,
            refresh_lifetime,
            ..Default::default()
        };

        // Encoding key
        match (secret, private_key, jwt.use_secret()) {
            (Some(secret), _, true) => jwt.set_encoding_key(secret.trim())?,
            (_, Some(private_key), false) => jwt.set_encoding_key(private_key.trim())?,
            _ => {
                return Err(ApiError::InternalError {
                    message: "invalid JWT encoding key".to_owned(),
                })
            }
        }

        // Decoding key
        match (secret, public_key, jwt.use_secret()) {
            (Some(secret), _, true) => jwt.set_decoding_key(secret.trim())?,
            (_, Some(public_key), false) => jwt.set_decoding_key(public_key.trim())?,
            _ => {
                return Err(ApiError::InternalError {
                    message: "invalided JWT decoding key".to_owned(),
                })
            }
        }

        Ok(jwt)
    }

    /// Update access token lifetime (in minute)
    pub fn set_access_lifetime(&mut self, duration: i64) {
        self.access_lifetime = duration;
    }

    /// Update refresh token lifetime (in day)
    pub fn set_refresh_lifetime(&mut self, duration: i64) {
        self.refresh_lifetime = duration;
    }

    /// Encode key with the good algorithm
    pub fn encoding_key_from_str(algo: Algorithm, secret: &str) -> ApiResult<EncodingKey> {
        let key = match algo {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => EncodingKey::from_secret(secret.as_bytes()),
            Algorithm::ES256 | Algorithm::ES384 => {
                EncodingKey::from_ec_pem(secret.as_bytes()).map_err(|err| ApiError::InternalError {
                    message: err.to_string(),
                })?
            }
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => EncodingKey::from_rsa_pem(secret.as_bytes())
                .map_err(|err| ApiError::InternalError {
                    message: err.to_string(),
                })?,
            Algorithm::PS256 | Algorithm::PS384 | Algorithm::PS512 => EncodingKey::from_rsa_pem(secret.as_bytes())
                .map_err(|err| ApiError::InternalError {
                    message: err.to_string(),
                })?,
            Algorithm::EdDSA => EncodingKey::from_ed_pem(secret.as_bytes()).map_err(|err| ApiError::InternalError {
                message: err.to_string(),
            })?,
        };
        Ok(key)
    }

    /// Decode key with the good algorithm
    pub fn decoding_key_from_str(algo: Algorithm, secret: &str) -> ApiResult<DecodingKey> {
        let key = match algo {
            Algorithm::HS256 | Algorithm::HS384 | Algorithm::HS512 => DecodingKey::from_secret(secret.as_bytes()),
            Algorithm::ES256 | Algorithm::ES384 => {
                DecodingKey::from_ec_pem(secret.as_bytes()).map_err(|err| ApiError::InternalError {
                    message: err.to_string(),
                })?
            }
            Algorithm::RS256 | Algorithm::RS384 | Algorithm::RS512 => DecodingKey::from_rsa_pem(secret.as_bytes())
                .map_err(|err| ApiError::InternalError {
                    message: err.to_string(),
                })?,
            Algorithm::PS256 | Algorithm::PS384 | Algorithm::PS512 => DecodingKey::from_rsa_pem(secret.as_bytes())
                .map_err(|err| ApiError::InternalError {
                    message: err.to_string(),
                })?,
            Algorithm::EdDSA => DecodingKey::from_ed_pem(secret.as_bytes()).map_err(|err| ApiError::InternalError {
                message: err.to_string(),
            })?,
        };
        Ok(key)
    }

    /// Update encoding key
    pub fn set_encoding_key(&mut self, secret: &str) -> ApiResult<()> {
        self.encoding_key = Some(Self::encoding_key_from_str(self.algorithm, secret)?);
        Ok(())
    }

    /// Update decoding key
    pub fn set_decoding_key(&mut self, secret: &str) -> ApiResult<()> {
        self.decoding_key = Some(Self::decoding_key_from_str(self.algorithm, secret)?);
        Ok(())
    }

    /// Generate JWT
    pub fn generate(&self, user_id: String, scopes: Vec<AuthScope>) -> ApiResult<(String, i64)> {
        let header = jsonwebtoken::Header::new(self.algorithm);
        let now = Utc::now().timestamp();
        let access_expired_at = now + (self.access_lifetime * 60);

        let payload = Claims {
            sub: user_id.clone(),
            exp: access_expired_at,
            iat: now,
            nbf: now,
            user_id,
            scopes,
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

                Ok((token, access_expired_at))
            }
            _ => Err(api_error!(
                ApiErrorCode::InternalError,
                "error during JWT encoding",
                "error during JWT encoding: no encoding key"
            )),
        }
    }

    /// Parse JWT
    pub fn parse(&self, token: &str) -> ApiResult<Claims> {
        let validation = Validation::new(self.algorithm);
        match self.decoding_key.clone() {
            Some(decoding_key) => {
                let token = decode::<Claims>(token, &decoding_key, &validation).map_err(|err| match err.kind() {
                    ExpiredSignature => api_error!(ApiErrorCode::Unauthorized),
                    _ => api_error!(ApiErrorCode::InternalError),
                })?;

                Ok(token.claims)
            }
            _ => Err(api_error!(
                ApiErrorCode::InternalError,
                "error during JWT decoding",
                "error during JWT decoding: no decoding key"
            )),
        }
    }

    /// Get user id from token
    pub fn user_id(&self, token: Option<String>) -> Option<String> {
        match token {
            Some(token) => self.parse(&token).ok().map(|c| c.user_id),
            None => None,
        }
    }
}
