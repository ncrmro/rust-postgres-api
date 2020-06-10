#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

pub mod core;
pub mod todo;
pub mod user;

use anyhow::Result;
use paperclip::actix::{api_v2_operation, web};

#[api_v2_operation]
async fn index() -> Result<String, ()> {
    Ok("Hello World".to_string())
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    let v1 = web::scope("/v1")
        .configure(todo::init)
        .configure(user::init);

    cfg.route("/", web::get().to(index)).service(v1);
}

#[actix_rt::main]
pub async fn init() -> Result<()> {
    info!("Starting server");
    let settings = core::settings::Settings::new().unwrap();
    core::http::server(settings, routes).await?;
    Ok(())
}
