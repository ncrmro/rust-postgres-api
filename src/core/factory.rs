use async_trait::async_trait;

use super::db;

#[async_trait]
pub trait ModelFactory<Model, NewModel> {
    fn build() -> NewModel;
    async fn create(obj: Option<NewModel>, pool: db::PgPool) -> Result<Model, db::Error>;
}
