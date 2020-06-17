use anyhow::Result;

use super::User;
use crate::core::auth::ViewerModel;
use crate::core::db::PgPool;
use crate::core::http;
use crate::core::http::api_v2_operation;
use crate::core::http::errors::*;
use crate::core::http::Apiv2Schema;

use crate::core::db::model::DatabaseModel;

#[derive(Serialize, Deserialize, Clone, Apiv2Schema)]
pub struct AuthenticationResponse {
    pub token: String,
    pub user: super::User,
}

#[api_v2_operation]
async fn viewer(
    _req: http::HttpRequest,
    viewer: super::Viewer,
    db_pool: http::Data<PgPool>,
) -> Result<http::Json<User>, Error> {
    if let Some(id) = viewer.id {
        let viewer = User::get(id, &db_pool).await;
        match viewer {
            Ok(user) => Ok(http::Json(user)),
            Err(_err) => Err(ErrorBadRequest("Not Authenticated")),
        }
    } else {
        Err(ErrorBadRequest("Not Authenticated"))
    }
}

#[api_v2_operation]
async fn create(
    user: http::Json<super::NewUser>,
    db_pool: http::Data<PgPool>,
) -> Result<http::Json<AuthenticationResponse>, Error> {
    let res = super::User::create_user(user.into_inner(), db_pool.get_ref()).await;
    match res {
        Ok(user) => {
            let res = AuthenticationResponse {
                user: user.clone(),
                token: super::utils::jwt_get(user.id),
            };
            Ok(http::Json(res))
        }
        Err(_e) => Err(ErrorConflict("user-exists")),
    }
}

#[api_v2_operation]
async fn authenticate(
    user: http::Json<super::LoginUser>,
    db_pool: http::Data<PgPool>,
) -> Result<http::Json<AuthenticationResponse>, Error> {
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
        Ok(http::Json(res))
    } else {
        Err(ErrorBadRequest("Not Authenticated"))
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut http::ServiceConfig) {
    cfg.route("/viewer", http::get().to(viewer))
        .route("/viewer/create", http::post().to(create))
        .route("/viewer/authenticate", http::post().to(authenticate));
}
