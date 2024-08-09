pub mod dto;
pub(crate) mod extractors;
pub(crate) mod handlers;
pub mod layers;
pub mod logger;
pub mod routes;
pub mod server;
pub mod use_cases;

extern crate chrono;
extern crate serde;

use std::sync::LazyLock;
use tera::Tera;

pub static TEMPLATES: LazyLock<Result<Tera, tera::Error>> = LazyLock::new(|| {
    let mut tera = Tera::new("templates/**/*")?;
    tera.autoescape_on(vec![".html", ".txt"]);
    Ok(tera)
});
