//! Error entity module

use thiserror::Error;

#[derive(Debug, Error)]
pub enum UserError {
    #[error("user error")]
    Custom,
}
