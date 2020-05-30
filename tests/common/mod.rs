use actix_http::Request;
use actix_service;
use actix_web::dev::ServiceResponse;
use actix_web::{test, App, Error};

use paperclip::actix::OpenApiExt;

mod db;

pub async fn server(
) -> impl actix_service::Service<Request = Request, Response = ServiceResponse, Error = Error> {
    let settings = planet_express::settings::Settings::new().unwrap();
    let db_pool = db::init(settings).await;

    let app = App::new()
        .data(db_pool.clone()) // pass database pool to application so we can access it inside handlers
        .wrap_api()
        .configure(planet_express::http::routes)
        .build();

    test::init_service(app).await
}
