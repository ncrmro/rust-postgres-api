use actix_http::Request;
use actix_service;
use actix_web::dev::ServiceResponse;
use actix_web::{test, App, Error};

use paperclip::actix::OpenApiExt;

pub async fn server(
) -> impl actix_service::Service<Request = Request, Response = ServiceResponse, Error = Error> {
    let app = App::new()
        .wrap_api()
        .configure(planet_express::http::routes)
        .build();

    test::init_service(app).await
}
