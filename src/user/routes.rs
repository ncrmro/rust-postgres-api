use crate::user::User;
use paperclip::actix::web::{self, get, Json};
use sqlx::PgPool;

async fn find(username: String, db_pool: web::Data<PgPool>) -> Result<Json<User>, ()> {
    let result = User::find_by_username(username, db_pool.get_ref()).await;
    match result {
        Ok(user) => Ok(Json(user)),
        _ => Err(()),
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/viewer", get().to(find))
        .route("/viewer2", get().to(find));
}
