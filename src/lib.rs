#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod db;
pub mod http;
pub mod settings;
pub mod todo;
pub mod user;

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
