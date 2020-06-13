use anyhow::Result;

use super::User;
use crate::core::auth::ViewerModel;
use crate::core::db::PgPool;
use crate::core::http::api_v2_operation;
use crate::core::http::errors::*;
use crate::core::http::web;
use crate::core::http::Apiv2Schema;

use crate::core::db::model::DatabaseModel;

#[derive(Serialize, Deserialize, Clone, Apiv2Schema)]
pub struct AuthenticationResponse {
    pub token: String,
    pub user: super::User,
}

#[api_v2_operation]
async fn viewer(
    _req: web::HttpRequest,
    viewer: super::Viewer,
    db_pool: web::Data<PgPool>,
) -> Result<web::Json<User>, Error> {
    if let Some(id) = viewer.id {
        let viewer = User::get(id, &db_pool).await;
        match viewer {
            Ok(user) => Ok(web::Json(user)),
            Err(_err) => Err(ErrorBadRequest("Not Authenticated")),
        }
    } else {
        Err(ErrorBadRequest("Not Authenticated"))
    }
}

#[api_v2_operation]
async fn create(
    user: web::Json<super::NewUser>,
    db_pool: web::Data<PgPool>,
) -> Result<web::Json<AuthenticationResponse>, Error> {
    let res = super::User::create_user(user.into_inner(), db_pool.get_ref()).await;
    match res {
        Ok(user) => {
            let res = AuthenticationResponse {
                user: user.clone(),
                token: super::utils::jwt_get(user.id),
            };
            Ok(web::Json(res))
        }
        Err(_e) => Err(ErrorConflict("user-exists")),
    }
}

#[api_v2_operation]
async fn authenticate(
    user: web::Json<super::LoginUser>,
    db_pool: web::Data<PgPool>,
) -> Result<web::Json<AuthenticationResponse>, Error> {
    let row = super::User::find_user_by_credentials(
        user.email.clone(),
        user.password.clone(),
        db_pool.get_ref(),
    )
    .await
    .unwrap();

    if let Some(user) = row {
        let res = AuthenticationResponse {
            token: super::utils::jwt_get(user.id),
            user,
        };
        Ok(web::Json(res))
    } else {
        Err(ErrorBadRequest("Not Authenticated"))
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/viewer", web::get().to(viewer))
        .route("/viewer/create", web::post().to(create))
        .route("/viewer/authenticate", web::post().to(authenticate));
}
