use crate::user::{AuthResponse, User, UserAuth};
use actix_http::error::{Error, ErrorBadRequest};
use actix_web::web::Data;
use paperclip::actix::web::HttpRequest;
use paperclip::actix::{api_v2_operation, web};
use sqlx::PgPool;

#[api_v2_operation]
async fn viewer(
    _req: HttpRequest,
    viewer: Option<User>,
    _db_pool: web::Data<PgPool>,
) -> Result<web::Json<User>, Error> {
    match viewer {
        Some(user) => Ok(web::Json(user)),
        None => Err(ErrorBadRequest("Not Authenticated")),
    }
}

#[api_v2_operation]
async fn create(
    user: web::Json<UserAuth>,
    db_pool: Data<PgPool>,
) -> Result<web::Json<AuthResponse>, ()> {
    let user = User::create(&user.into_inner(), db_pool.get_ref())
        .await
        .unwrap();
    let res = AuthResponse {
        token: super::auth::jwt_get(user.id),
        user,
    };
    Ok(web::Json(res))
}

#[api_v2_operation]
async fn authenticate(
    user: web::Json<UserAuth>,
    db_pool: Data<PgPool>,
) -> Result<web::Json<AuthResponse>, ()> {
    let user = User::authenticate(user.into_inner(), db_pool.get_ref())
        .await
        .unwrap();
    let res = AuthResponse {
        token: super::auth::jwt_get(user.id),
        user,
    };
    Ok(web::Json(res))
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    // .route("/viewer", web::get().to(viewer))
    cfg.route("/viewer", web::get().to(viewer))
        .route("/viewer/create", web::post().to(create))
        .route("/viewer/authenticate", web::post().to(authenticate));
}
