use async_trait::async_trait;

use crate::core::db;
use crate::core::db::model::{fields, DatabaseModel};
use crate::core::db::Error;
use crate::core::db::FromRow;
use crate::core::db::PgPool;
use crate::core::http::Apiv2Schema;

pub type UserId = fields::Id;

type RepoError = Error;

#[derive(Serialize, Deserialize, FromRow, Apiv2Schema, Clone)]
pub struct User {
    pub id: UserId,
    pub email: String,
    pub password: String,
    pub image: Option<String>,
    pub created_at: fields::DateTime,
    pub updated_at: fields::DateTime,
}

#[derive(Serialize, Deserialize, Clone, Apiv2Schema)]
pub struct NewUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Apiv2Schema)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone, Apiv2Schema)]
pub struct CurrentUser {
    pub username: String,
}

#[async_trait]
impl DatabaseModel<User, NewUser> for User {
    async fn create(_obj: NewUser, _db_conn: &PgPool) -> Result<User, Error> {
        unimplemented!()
    }

    async fn get(id: i32, db_conn: &PgPool) -> Result<User, db::Error> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
            .fetch_one(db_conn)
            .await
    }

    // async fn find_by_field<T>(field: String, value: T, db_conn: &PgPool) -> Result<User, Error> {
    //         sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
    //         .fetch_one(db_conn)
    //         .await
    // }
}

#[async_trait]
pub trait ViewerModel<Model, NewModel>: DatabaseModel<Model, NewModel> {
    async fn create_user(obj: NewUser, conn: &PgPool) -> Result<Model, db::Error>;
    async fn find_user_by_credentials(
        email: String,
        password: String,
        db_conn: &PgPool,
    ) -> Result<Option<Model>, RepoError>;
}

#[async_trait]
impl ViewerModel<User, NewUser> for User {
    async fn create_user(obj: NewUser, conn: &PgPool) -> Result<User, db::Error> {
        sqlx::query_as!(
            User,
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *",
            obj.email,
            super::hash_password(obj.password),
        )
        .fetch_one(conn)
        .await
    }

    async fn find_user_by_credentials(
        email: String,
        password: String,
        db_conn: &PgPool,
    ) -> Result<Option<Self>, RepoError> {
        let rec = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_one(db_conn)
            .await;
        match rec {
            Ok(user) => {
                if super::verify_password(user.password.clone(), password) {
                    Ok(Some(user))
                } else {
                    Ok(None)
                }
            }
            Err(e) => Err(e),
        }
    }
}
