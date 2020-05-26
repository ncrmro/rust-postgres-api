#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

use crate::http::server;
use anyhow::Result;

mod settings;
mod http;
mod db;

use settings::Settings;

#[actix_rt::main]
async fn main() -> Result<()> {
    let settings = Settings::new().unwrap();
    info!("Starting server");
    server(settings).await?;
    Ok(())
}
