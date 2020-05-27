use crate::db::init_db;
use crate::settings::Settings;
// use crate::user;
use actix_web::{App, HttpServer};
use paperclip::actix::{
    // extension trait for actix_web::App and proc-macro attributes
    OpenApiExt, api_v2_operation,
    // use this instead of actix_web::web
    web::{self},
};
use anyhow::Result;
use listenfd::ListenFd;


#[api_v2_operation]
async fn index() -> Result<String, () > {
    Ok("Hello World".to_string())
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
            .service( web::resource("/").route(web::get().to(index)) )
//             .configure(user::init) // init todo routes
            .with_json_spec_at("/api/spec")
            .build()
    });

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(format!("{}:{}", settings.http.host, settings.http.port))?
    };

    Ok(server.run().await?)
}
