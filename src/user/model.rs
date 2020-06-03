use actix_web::error::ErrorBadRequest;
use actix_web::{dev, Error as ActixError, FromRequest, HttpRequest, HttpResponse, Responder};

use anyhow::Result;
use fake::{faker::internet, Fake};

use futures::future::{err, ok, ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::{Error, PgPool};

use crate::user::auth::jwt_verify;
use argon2::{self, Config};
use paperclip::actix::Apiv2Schema;
use std::borrow::Borrow;

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct UserAuth {
    pub id: Option<i32>,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct User {
    pub id: i32,
    pub email: String,
}

impl FromRequest for User {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let ext = req.extensions();
        let user = ext.get::<User>();
        if let Some(user) = user {
            ok(User {
                id: user.id,
                email: user.email.to_string(),
            })
        } else {
            err(ErrorBadRequest("DICKHOLE"))
        }
    }
}

#[derive(Serialize, Deserialize, Apiv2Schema)]
pub struct AuthResponse {
    pub token: String,
    pub user: User,
}

impl Responder for User {
    type Error = ActixError;
    type Future = Ready<Result<HttpResponse, ActixError>>;

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
    pub async fn verify_token(token: String, conn: &PgPool) -> Result<User, bool> {
        let claims = jwt_verify(token.parse().unwrap());
        if claims.is_err() {
            return Err(false);
        }
        let row = sqlx::query!(
            "SELECT id, email, password FROM users WHERE id = $1",
            claims.unwrap().user_id
        )
        .fetch_one(conn)
        .await;

        match row {
            Ok(user) => Ok(User {
                id: user.id,
                email: user.email,
            }),
            Err(_err) => Err(false),
        }
    }

    pub async fn authenticate(obj: UserAuth, conn: &PgPool) -> Result<User> {
        let email = obj.email;
        let rec = sqlx::query!(
            "SELECT id, email, password FROM users WHERE email = $1",
            email
        )
        .fetch_one(conn)
        .await?;
        let matches = argon2::verify_encoded(&rec.password, obj.password.as_ref()).unwrap();
        assert!(matches);
        let user = User {
            id: rec.id,
            email: rec.email,
        };
        Ok(user)
    }
    pub async fn create(obj: &UserAuth, conn: &PgPool) -> Result<User, Error> {
        let salt = b"randomsalt";
        let hash = argon2::hash_encoded(obj.password.as_ref(), salt, &Config::default()).unwrap();
        let rec = sqlx::query!(
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id, email",
            obj.email,
            hash
        )
        .fetch_one(conn)
        .await;

        match rec {
            Ok(rec) => Ok(User {
                id: rec.id,
                email: rec.email.to_string(),
            }),
            Err(e) => err(e).await,
        }
    }
}

pub struct UserFactory {}

impl UserFactory {
    pub fn build() -> UserAuth {
        UserAuth {
            id: None,
            email: internet::en::FreeEmail().fake(),
            password: "testpassword".to_string(),
        }
    }

    pub async fn create(pool: PgPool) -> UserAuth {
        let obj_build = UserFactory::build();
        let obj = User::create(&obj_build, pool.borrow()).await.unwrap();
        UserAuth {
            id: None,
            email: obj.email,
            password: obj_build.password,
        }
    }
}
