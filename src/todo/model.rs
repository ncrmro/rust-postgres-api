use async_trait::async_trait;

use anyhow::Result;

use crate::core::db;
use crate::core::db::model::DatabaseModel;
use crate::core::db::FromRow;
use crate::core::http::Apiv2Schema;

// this struct will use to receive user input
#[derive(Serialize, Deserialize, Apiv2Schema, Clone)]
pub struct TodoRequest {
    pub description: String,
    pub done: bool,
}

// this struct will be used to represent database record
#[derive(Serialize, Deserialize, FromRow, Apiv2Schema, Clone, Debug)]
pub struct Todo {
    pub id: i32,
    pub description: String,
    pub done: bool,
}

// implementation of Actix Responder for Todo struct so we can return Todo from action handler

// Implementation for Todo struct, functions for read/write/update and delete todo from database
#[async_trait]
impl DatabaseModel<Todo, TodoRequest> for Todo {
    async fn create(obj: TodoRequest, conn: &db::PgPool) -> Result<Self, db::Error> {
        let rec = sqlx::query_as!(
            Todo,
            "INSERT INTO todos (description, done) VALUES ($1, $2) RETURNING *",
            obj.description,
            false
        )
        .fetch_one(conn)
        .await;

        match rec {
            Ok(part) => Ok(part),
            Err(e) => Err(e),
        }
    }

    async fn get(id: i32, db_conn: &db::PgPool) -> Result<Todo, db::Error> {
        let rec = sqlx::query!(
            r#"
                    SELECT * FROM todos WHERE id = $1
                "#,
            id
        )
        .fetch_one(&*db_conn)
        .await?;

        Ok(Todo {
            id: rec.id,
            description: rec.description,
            done: rec.done,
        })
    }
}
