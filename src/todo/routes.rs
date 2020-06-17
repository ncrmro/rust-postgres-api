use futures::future::{ready, Ready};

use super::model;
use crate::core::db::model::DatabaseModel;
use crate::core::db::PgPool;
use crate::core::http;
use crate::core::http::api_v2_operation;
use crate::core::http::errors::Error;
use crate::core::http::HttpRequest;
use crate::core::http::HttpResponse;
use crate::core::http::Json;
use crate::core::http::Responder;

impl Responder for model::Todo {
    type Error = Error;
    type Future = Ready<Result<HttpResponse, Error>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        let body = serde_json::to_string(&self).unwrap();
        // create response and set content type
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(body)))
    }
}

#[api_v2_operation]
async fn find(
    id: http::Path<i32>,
    db_pool: http::Data<PgPool>,
) -> Result<Json<super::model::Todo>, ()> {
    let result = model::Todo::get(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(todo) => Ok(Json(todo)),
        Err(_e) => Err(()),
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut http::ServiceConfig) {
    cfg.route("/todo/{id}", http::get().to(find));
}
