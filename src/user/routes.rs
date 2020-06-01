use crate::user::{AuthResponse, User, UserAuth};
use paperclip::actix::{api_v2_operation, web};
use sqlx::PgPool;

// #[api_v2_operation]
// async fn viewer(req: HttpRequest, db_pool: web::Data<PgPool>) -> Result<web::Json<User>, ()> {
//     let result = User::find_by_username(username, db_pool.get_ref()).await;
//     match result {
//         Ok(user) => Ok(web::Json(user)),
//         _ => Err(()),
//     }
// }

#[api_v2_operation]
async fn create(
    user: web::Json<UserAuth>,
    db_pool: web::Data<PgPool>,
) -> Result<web::Json<AuthResponse>, ()> {
    let res = User::create(&user.into_inner(), db_pool.get_ref())
        .await
        .unwrap();

    Ok(web::Json(res))
}

#[api_v2_operation]
async fn authenticate(
    user: web::Json<UserAuth>,
    db_pool: web::Data<PgPool>,
) -> Result<web::Json<AuthResponse>, ()> {
    let res = User::authenticate(user.into_inner(), db_pool.get_ref())
        .await
        .unwrap();
    Ok(web::Json(res))
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    // .route("/viewer", web::get().to(viewer))
    cfg.route("/viewer/create", web::post().to(create))
        .route("/viewer/authenticate", web::post().to(authenticate));
}
