use actix_web::error::ErrorBadRequest;
use actix_web::{dev, Error as ActixError, FromRequest, HttpRequest, HttpResponse, Responder};

use anyhow::Result;
use fake::{faker::internet, Fake};

use futures::future::{err, ok, ready, Ready};
use serde::{Deserialize, Serialize};
use sqlx::{Error, FromRow, PgPool};

use crate::user::auth::jwt_verify;
use argon2::{self, Config};
use paperclip::actix::Apiv2Schema;

use sqlx::types::chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, Apiv2Schema)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
    pub image: Option<String>,
}

#[derive(Serialize, Deserialize, FromRow, Apiv2Schema, Debug, Clone)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl FromRequest for User {
    type Error = ActixError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _payload: &mut dev::Payload) -> Self::Future {
        let ext = req.extensions();
        let user = ext.get::<User>();
        if let Some(user) = user {
            ok(user.clone())
        } else {
            err(ErrorBadRequest("DICKHOLE"))
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Apiv2Schema)]
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
        let rec = User::get_by_id(claims.unwrap().user_id, conn).await;

        match rec {
            Ok(user) => Ok(user),
            Err(_err) => Err(false),
        }
    }

    pub async fn authenticate(obj: UserRequest, conn: &PgPool) -> Result<User, &str> {
        #[derive(Debug, Clone)]
        struct DoubleError;

        let query = User::get_by_email(obj.email, conn).await;
        if query.is_err() {
            return Err("User not found");
        }
        let user = query.unwrap();
        if argon2::verify_encoded(&user.password, obj.password.as_ref()).is_err() {
            return Err("Password is incorrect");
        }

        Ok(user)
    }
    pub async fn create(obj: UserRequest, conn: &PgPool) -> Result<User, Error> {
        let salt = b"randomsalt";
        let hash = argon2::hash_encoded(obj.password.as_ref(), salt, &Config::default()).unwrap();
        let rec = sqlx::query_as!(
            User,
            "INSERT INTO users (email, image, password) VALUES ($1, $2, $3) RETURNING *",
            obj.email,
            obj.image,
            hash,
        )
        .fetch_one(conn)
        .await;
        match rec {
            Ok(user) => Ok(user),
            Err(e) => {
                return Err(e);
            }
        }
    }

    pub async fn get_by_id(id: i32, pool: &PgPool) -> Result<User, Error> {
        let rec = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&*pool)
            .await?;
        Ok(rec)
    }

    pub async fn get_by_email(email: String, pool: &PgPool) -> Result<User, Error> {
        let rec = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(&*pool)
            .await?;
        Ok(rec)
    }
}

pub struct UserFactory {}

impl UserFactory {
    pub fn build() -> UserRequest {
        UserRequest {
            email: internet::en::FreeEmail().fake(),
            password: "testpassword".to_string(),
            image: None,
        }
    }

    pub async fn create(pool: PgPool) -> User {
        let obj_build = UserFactory::build();
        User::create(obj_build, &pool).await.unwrap()
    }
}
