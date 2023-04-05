pub mod config;
pub(crate) mod extractors;
pub(crate) mod handlers;
pub mod layers;
pub mod logger;
pub mod routes;
pub mod server;
pub mod usecases;

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
