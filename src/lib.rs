#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

mod db;
mod http;
mod settings;
mod user;
mod todo;

use settings::Settings;
use http::server;
use anyhow::Result;

#[actix_rt::main]
pub async fn init() -> Result<()> {
    let settings = Settings::new().unwrap();
    info!("Starting server");
    server(settings).await?;
    Ok(())
}
