pub mod middlewares;

use crate::core::db::init_db;
use crate::core::settings::Settings;
use crate::todo;
use crate::user;
use crate::user::User;
use actix_web::middleware::Logger;
use actix_web::{guard, App, HttpServer};
use anyhow::Result;
use chrono::prelude::*;
use env_logger::Env;
use listenfd::ListenFd;
use paperclip::actix::{
    api_v2_operation,
    web::{self},
    OpenApiExt,
};

#[api_v2_operation]
async fn index() -> Result<String, ()> {
    Ok("Hello World".to_string())
}

#[api_v2_operation]
async fn p404() -> Result<String, ()> {
    Ok("404".to_string())
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    let v1 = web::scope("/v1")
        .configure(todo::init)
        .configure(user::init);

    cfg.route("/", web::get().to(index)).service(v1);
}

pub async fn server(settings: Settings) -> Result<()> {
    // this will enable us to keep application running during recompile: systemfd --no-pid -s http::5000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

    let db_pool = init_db(&settings.database).await?;

    env_logger::from_env(Env::default().default_filter_or("info")).init();

    let mut server = HttpServer::new(move || {
        App::new()
            // Record services and routes from this line.
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .data(User {
                id: 0,
                email: "".to_string(),
                password: "".to_string(),
                image: None,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            })
            .wrap(Logger::default())
            .wrap(middlewares::Viewer)
            .wrap_api()
            .with_json_spec_at("/api/spec.json")
            .configure(routes)
            .build()
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not `GET`
                    .route(web::Route::new().guard(guard::Not(guard::Get()))),
            )
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(format!("{}:{}", "0.0.0.0", settings.http.port))?,
    };

    Ok(server.run().await?)
}
