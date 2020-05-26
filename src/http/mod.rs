use listenfd::ListenFd;
use std::env;
use actix_web::{get, App, HttpServer, Responder};
use crate::settings::Settings;
use sqlx::postgres::PgPool;
use anyhow::Result;

#[get("/")]
async fn index() -> impl Responder {
    "Hello World"
}

pub async fn server(settings: Settings) -> Result<()> {
    // this will enable us to keep application running during recompile: systemfd --no-pid -s http::5000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPool::new(&database_url).await?;

    let mut server = HttpServer::new(move || {
        App::new()
        .data( db_pool.clone()  ) // pass database pool to application so we can access it inside handlers
        .service(index)
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("HOST is not set in .env file");
            let port = env::var("PORT").expect("PORT is not set in .env file");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    Ok(server.run().await?)
}

