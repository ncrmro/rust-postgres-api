use crate::db::init_db;
use crate::settings::Settings;
use crate::todo;
use crate::user;
use actix_web::{App, HttpServer};
use anyhow::Result;
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

    let mut server = HttpServer::new(move || {
        App::new()
            // Record services and routes from this line.
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .wrap_api()
            .with_json_spec_at("/api/spec")
            .configure(routes)
            .build()
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(format!("{}:{}", "0.0.0.0", settings.http.port))?,
    };

    Ok(server.run().await?)
}
