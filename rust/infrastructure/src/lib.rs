pub mod api;
pub mod cli;
pub mod config;
pub mod database;
pub mod email;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate tracing;

/// Application name
pub const APP_NAME: &str = "Backend Clean Architecture with Rust";
