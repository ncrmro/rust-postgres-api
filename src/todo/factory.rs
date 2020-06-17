use async_trait::async_trait;

use super::model::{Todo as TodoModel, TodoRequest};
use crate::core::db;
use crate::core::db::model::DatabaseModel;
use crate::core::factory::ModelFactory;
use fake::{faker::internet, Fake};

pub struct Todo;

#[async_trait]
impl ModelFactory<TodoModel, TodoRequest> for Todo {
    fn build() -> TodoRequest {
        TodoRequest {
            description: "".to_string(),
            done: false,
        }
    }

    async fn create(
        request: Option<TodoRequest>,
        conn: db::PgPool,
    ) -> Result<TodoModel, db::Error> {
        let obj_build = request.unwrap_or_else(Self::build);
        TodoModel::create(obj_build, &conn).await
    }
}
