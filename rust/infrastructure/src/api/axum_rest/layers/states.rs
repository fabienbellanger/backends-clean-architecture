//! States

use crate::config::Config;
use clean_architecture_shared::{auth::Jwt, error::ApiResult};
use std::{fs::read_to_string, sync::Arc};

/// SharedState
pub type SharedState = Arc<State>;

// #[derive(Default, Debug)]
pub struct State {
    pub config: ConfigState,
    pub jwt: Jwt,
}

impl State {
    /// Initialize `State` with configuration data (`.env`)
    pub fn init(config: &Config) -> ApiResult<Self> {
        let config_state: ConfigState = config.clone().into();
        let private_key = match config.jwt_private_key.as_deref() {
            Some(name) => read_to_string(format!("./keys/{}", name)).ok(),
            None => None,
        };
        let public_key = match config.jwt_public_key.as_deref() {
            Some(name) => read_to_string(format!("./keys/{}", name)).ok(),
            None => None,
        };
        let jwt = Jwt::new(
            &config.jwt_algorithm,
            config.jwt_access_lifetime,
            config.jwt_refresh_lifetime,
            config.jwt_secret_key.as_deref(),
            private_key.as_deref(),
            public_key.as_deref(),
        )?;

        Ok(Self {
            config: config_state,
            jwt,
        })
    }
}

pub struct ConfigState {
    pub forgotten_password_expiration_duration: i64,
}

impl From<Config> for ConfigState {
    fn from(config: Config) -> Self {
        Self {
            forgotten_password_expiration_duration: config.forgotten_password_expiration_duration,
        }
    }
}
