#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod db;
pub mod http;
mod settings;
mod todo;
mod user;

use anyhow::Result;
use http::server;
use settings::Settings;

#[actix_rt::main]
pub async fn init() -> Result<()> {
    let settings = Settings::new().unwrap();
    info!("Starting server");
    server(settings).await?;
    Ok(())
}
