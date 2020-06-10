use async_trait::async_trait;

use super::db;

#[async_trait]
pub trait ModelFactory {
    type Request;
    type Model;

    fn factory_build() -> Self::Request;
    async fn factory_create(
        obj: Option<Self::Request>,
        pool: db::PgPool,
    ) -> Result<Self::Model, db::Error>;
}
