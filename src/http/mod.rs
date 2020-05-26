use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello World"
}

pub async fn server() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}

