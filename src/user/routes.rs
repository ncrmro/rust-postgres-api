use crate::user::User;
use crate::user::UserRequest;
use paperclip::actix::{api_v2_operation, web};
use sqlx::PgPool;

// #[api_v2_operation]
// async fn find(username: String, db_pool: web::Data<PgPool>) -> Result<Json<User>, ()> {
//     let result = User::find_by_username(username, db_pool.get_ref()).await;
//     match result {
//         Ok(user) => Ok(Json(user)),
//         _ => Err(()),
//     }
// }

#[api_v2_operation]
async fn create(
    user: web::Json<UserRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<web::Json<User>, ()> {
    let obj = User::create(user.into_inner(), db_pool.get_ref())
        .await
        .unwrap();

    println!("CREATE MOFO");
    Ok(web::Json(obj))
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    // cfg.route("/viewer", get().to(find))
    cfg.route("/viewer/create", web::post().to(create));
}
