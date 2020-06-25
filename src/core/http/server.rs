use actix_web::App;
use actix_web::HttpServer;
use anyhow::Result;
use listenfd::ListenFd;
use paperclip::actix::OpenApiExt;

use crate::core::auth::Viewer;
use crate::core::db::init_db;
use crate::core::settings::Settings;

use super::api_v2_operation;
use super::guard;
use super::middlewares;

#[api_v2_operation]
async fn p404() -> Result<String, super::errors::Error> {
    Err(super::errors::ErrorNotFound("Route not found"))
}

pub async fn server(settings: Settings, routes: fn(&mut super::ServiceConfig)) -> Result<()> {
    info!("Initializing server");

    // this will enable us to keep application running during recompile: systemfd --no-pid -s http::5000 -- cargo watch -x run
    let mut listenfd = ListenFd::from_env();

    let db_pool = init_db(&settings.database).await?;
    let viewer = Viewer { id: None };
    let mut server = HttpServer::new(move || {
        App::new()
            // Record services and routes from this line.
            .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
            .data(viewer)
            .wrap(middlewares::Logger::default())
            .wrap(middlewares::Viewer)
            .wrap_api()
            .with_json_spec_at("/api/spec.json")
            .configure(routes)
            .build()
            .default_service(
                // 404 for GET request
                super::resource("")
                    .route(super::get().to(p404))
                    // all requests that are not `GET`
                    .route(super::Route::new().guard(guard::Not(guard::Get()))),
            )
    });

    if settings.http.threads.is_some() {
        server = server.workers(settings.http.threads.unwrap() as usize);
    }

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind(format!(
            "{}:{}",
            settings.http.host.as_str(),
            settings.http.port
        ))?,
    };

    Ok(server.run().await?)
}
