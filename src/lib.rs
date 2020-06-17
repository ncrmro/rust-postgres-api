#![warn(clippy::style, clippy::correctness, clippy::complexity, clippy::perf)]
#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod core;
pub mod todo;
pub mod user;

use anyhow::Result;
use env_logger::Env;

use crate::core::http;
use crate::core::http::api_v2_operation;

#[api_v2_operation]
async fn index() -> Result<String, ()> {
    Ok("Hello World".to_string())
}

pub fn routes(cfg: &mut http::ServiceConfig) {
    let v1 = http::scope("/v1")
        .configure(todo::init)
        .configure(core::auth::init);

    cfg.route("/", http::get().to(index)).service(v1);
}

#[actix_rt::main]
pub async fn init() -> Result<()> {
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    let settings = core::settings::Settings::new().unwrap();
    core::http::server(settings, routes).await?;
    Ok(())
}
