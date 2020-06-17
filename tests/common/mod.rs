pub mod db;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::iter;

use actix_http::Request;
use actix_service::Service;
use actix_web::dev::ServiceResponse;
use actix_web::{test, App, Error};

use paperclip::actix::OpenApiExt;

use src::core::auth::Viewer;
use src::core::db::PgConnection;
use src::core::db::Pool;
use src::core::http::middlewares;
use src::core::settings::Settings;

pub async fn setup() -> (
    impl Service<Request = Request, Response = ServiceResponse, Error = Error>,
    Pool<PgConnection>,
    String,
) {
    let settings = Settings::new().unwrap();
    let mut rng = thread_rng();
    let test_name: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(7)
        .collect();
    let db_conn = db::init(settings, test_name.clone()).await;
    let viewer = Viewer { id: None };
    let app = App::new()
        .data(db_conn.clone()) // pass database pool to application so we can access it inside handlers
        .data(viewer)
        .wrap(middlewares::Viewer)
        .wrap_api()
        .configure(src::routes)
        .build();

    (test::init_service(app).await, db_conn, test_name)
}

pub async fn teardown(db_conn: Pool<PgConnection>, test_id: String) {
    let settings = Settings::new().unwrap();
    db_conn.close().await;
    db::down(settings, test_id).await;
}
