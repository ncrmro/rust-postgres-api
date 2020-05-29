use crate::todo::Todo;
use paperclip::actix::web::{self, get, Json};
use sqlx::PgPool;

async fn find(id: web::Path<i32>, db_pool: web::Data<PgPool>) -> Result<Json<Todo>, ()> {
    let result = Todo::find_by_id(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(todo) => Ok(Json(todo)),
        _ => Err(()),
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/todo", get().to(find))
        .route("/todo1", get().to(find));
}
