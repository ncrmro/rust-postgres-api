use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use paperclip::actix::Apiv2Schema;

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct UserRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct User {
    pub id: i32,
    pub email: String,
}

impl Responder for User {
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

// Implementation for User struct, functions for read/write/update and delete User from database
impl User {
    pub async fn find_by_username(username: String, pool: &PgPool) -> Result<User> {
        let rec = sqlx::query!(
            r#"
                    SELECT * FROM Users WHERE username = $1
                "#,
            username
        )
        .fetch_one(&*pool)
        .await?;

        Ok(User {
            id: rec.id,
            email: rec.email,
        })
    }
}
