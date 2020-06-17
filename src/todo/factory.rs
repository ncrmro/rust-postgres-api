use async_trait::async_trait;

use super::model::{Todo as TodoModel, TodoRequest};
use crate::core::db;
use crate::core::db::model::DatabaseModel;
use crate::core::factory::faker;
use crate::core::factory::Fake;
use crate::core::factory::ModelFactory;
use std::ops::Range;

pub struct Todo;

#[async_trait]
impl ModelFactory<TodoModel, TodoRequest> for Todo {
    fn build() -> TodoRequest {
        TodoRequest {
            description: faker::lorem::en::Sentence(Range { start: 0, end: 5 }).fake(),
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
