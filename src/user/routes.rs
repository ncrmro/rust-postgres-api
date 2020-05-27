use crate::user::User;
use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;

#[get("/viewer")]
async fn find(username: String, db_pool: web::Data<PgPool>) -> impl Responder {
    let result = User::find_by_username(username, db_pool.get_ref()).await;
    match result {
        Ok(todo) => HttpResponse::Ok().json(todo),
        _ => HttpResponse::BadRequest().body("Todo not found"),
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(find);
}
