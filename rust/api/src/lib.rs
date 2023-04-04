pub mod config;
pub(crate) mod extractors;
pub(crate) mod handlers;
pub mod layers;
pub(crate) mod logger;
pub mod routes;
pub mod server;
pub(crate) mod usecases;

#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate tracing;

extern crate chrono;
extern crate serde;

use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Result<Tera, tera::Error> = {
        let mut tera = Tera::new("templates/**/*")?;
        tera.autoescape_on(vec![".html", ".txt"]);
        Ok(tera)
    };
}
