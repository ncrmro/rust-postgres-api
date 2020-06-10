#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod core;
pub mod todo;
pub mod user;

use anyhow::Result;

#[actix_rt::main]
pub async fn init() -> Result<()> {
    info!("Starting server");
    let settings = core::settings::Settings::new().unwrap();
    core::http::server(settings).await?;
    Ok(())
}
