use actix_web::{Error, HttpRequest, HttpResponse, Responder};
use anyhow::Result;
use fake::{faker::internet, Dummy, Fake, Faker};

use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};

use paperclip::actix::Apiv2Schema;

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
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
    pub async fn create(obj: UserRequest, pool: PgPool) -> Result<User> {
        let mut tx = pool.begin().await?;
        let obj = sqlx::query(
            "INSERT INTO users (username, password) VALUES ($1, $2) RETURNING id, username",
        )
        .bind(&obj.email)
        .bind(&obj.password)
        .map(|row: PgRow| User {
            id: row.get(0),
            email: row.get(1),
        })
        .fetch_one(&mut tx)
        .await?;

        tx.commit().await?;
        Ok(obj)
    }
}

pub struct UserFactory {
    saved: str,
}

impl UserFactory {
    pub fn build() -> UserRequest {
        UserRequest {
            email: internet::en::FreeEmail().fake(),
            password: "testpassword".to_string(),
        }
    }

    pub async fn new(pool: PgPool) -> User {
        User::create(UserFactory::build(), pool).await.unwrap()
    }
}
