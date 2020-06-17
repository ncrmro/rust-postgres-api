use futures::future::{ready, Ready};

use crate::core::db::PgPool;
use crate::todo::Todo;
use paperclip::actix::web::{self, get, Data, Json};

use crate::core::db::model::DatabaseModel;
use crate::core::http::errors::Error;
use crate::core::http::HttpRequest;
use crate::core::http::HttpResponse;
use crate::core::http::Responder;

impl Responder for Todo {
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

async fn find(id: web::Path<i32>, db_pool: Data<PgPool>) -> Result<Json<Todo>, ()> {
    let result = Todo::get(id.into_inner(), db_pool.get_ref()).await;
    match result {
        Ok(todo) => Ok(Json(todo)),
        _ => Err(()),
    }
}

// function that will be called on new Application to configure routes for this module
pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.route("/todo", get().to(find));
}
