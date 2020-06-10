use actix_http::Request;
use actix_service::Service;
use actix_web::dev::ServiceResponse;
use actix_web::{test, App, Error};

use paperclip::actix::OpenApiExt;
pub mod db;

use planet_express::core::settings::Settings;
use std::iter;

use chrono::Utc;
use planet_express::user::User;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use sqlx::{PgConnection, Pool};

async fn user() -> Result<User, ()> {
    Ok(User {
        id: 0,
        email: "".to_string(),
        password: "".to_string(),
        image: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    })
}
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

    let app = App::new()
        .data(db_conn.clone()) // pass database pool to application so we can access it inside handlers
        .data_factory(user)
        .wrap(planet_express::core::http::middlewares::Viewer)
        .wrap_api()
        .configure(planet_express::routes)
        .build();

    (test::init_service(app).await, db_conn, test_name)
}

pub async fn teardown(db_conn: Pool<PgConnection>, test_id: String) {
    let settings = Settings::new().unwrap();
    db_conn.close().await;
    db::down(settings, test_id).await;
}
