//! States module

use crate::config::Config;
use clean_architecture_shared::auth::Jwt;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey};
use std::sync::Arc;

/// SharedState
pub type SharedState = Arc<State>;

// #[derive(Default, Debug)]
pub struct State {
    pub config: ConfigState,
    pub jwt: Jwt,
}

impl State {
    /// Initialize `State` with configuration data (`.env`)
    pub fn init(config: &Config) -> Self {
        info!("Init app state");
        let config: ConfigState = config.clone().into();
        let jwt = Jwt::new(
            Algorithm::HS512,
            config.jwt_lifetime,
            Some(config.jwt_encoding_key.clone()),
            Some(config.jwt_decoding_key.clone()),
        );
        Self { config, jwt }
    }
}

// #[derive(Default, Debug)]
pub struct ConfigState {
    pub jwt_encoding_key: EncodingKey,
    pub jwt_decoding_key: DecodingKey,
    pub jwt_lifetime: i64,
}

impl From<Config> for ConfigState {
    fn from(config: Config) -> Self {
        Self {
            jwt_encoding_key: EncodingKey::from_secret(config.jwt_secret_key.as_bytes()),
            jwt_decoding_key: DecodingKey::from_secret(config.jwt_secret_key.as_bytes()),
            jwt_lifetime: config.jwt_lifetime,
        }
    }
}
