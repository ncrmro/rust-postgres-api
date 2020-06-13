// use crate::db::init_db;
use crate::settings::Settings;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn index() -> impl Responder {
    HttpResponse::Ok().body(r#"
        Welcome to Actix-web with SQLx Todos example.
        Available routes:
        GET /todos -> list of all todos
        POST /todo -> create new todo, example: { "description": "learn actix and sqlx", "done": false }
        GET /todo/{id} -> show one todo with requested id
        PUT /todo/{id} -> update todo with requested id, example: { "description": "learn actix and sqlx", "done": true }
        DELETE /todo/{id} -> delete todo with requested id
    "#
    )
}

pub async fn main(settings: Settings) -> std::result::Result<(), std::io::Error> {
    HttpServer::new(move || {
        App::new()
            //             .data(init_db(&settings.database)) // pass database pool to application so we can access it inside handlers
            .route("/", web::get().to(index))
        //             .configure(todo::init) // init todo routes
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
