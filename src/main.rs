#[macro_use]
extern crate log;

use crate::http::server;
mod http;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    info!("Starting server");
    server().await?;
    Ok(())
}
