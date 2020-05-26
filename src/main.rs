
use crate::http::server;
mod http;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server().await
}
