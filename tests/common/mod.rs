use actix_http::Request;
use actix_service::Service;
use actix_web::dev::ServiceResponse;
use actix_web::{test, App, Error};

use paperclip::actix::OpenApiExt;

pub mod db;
pub mod runner;

pub async fn server() -> impl Service<Request = Request, Response = ServiceResponse, Error = Error>
{
    let settings = planet_express::settings::Settings::new().unwrap();
    let connection = db::init(settings).await;

    let app = App::new()
        .data(connection) // pass database pool to application so we can access it inside handlers
        .wrap_api()
        .configure(planet_express::http::routes)
        .build();

    test::init_service(app).await
}
