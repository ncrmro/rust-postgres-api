use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

// this struct will use to receive user input
#[derive(Serialize, Deserialize)]
pub struct UserRequest {
    pub username: String,
}

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i32,
    pub email: String,
}

// implementation of Actix Responder for User struct so we can return User from action handler
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
