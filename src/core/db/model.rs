use async_trait::async_trait;

pub mod fields {
    pub type Id = i32;
    use sqlx::types::chrono::{DateTime as DT, Utc};
    pub type DateTime = DT<Utc>;
}

#[async_trait]
pub trait DatabaseModel<Model, NewModel> {
    async fn create(obj: NewModel, db_conn: &super::PgPool) -> Result<Model, super::Error>;
    async fn get(id: fields::Id, db_conn: &super::PgPool) -> Result<Model, super::Error>;
    // async fn update(obj: Self, db_conn: Conn) -> Result<Model, Err>;
    // async fn delete_by<Field>(field: Field, db_conn: Conn) -> Result<Model, Err>;
    // async fn find_by_field<T>(
    //     field: String,
    //     value: T,
    //     db_conn: &super::PgPool,
    // ) -> Result<Model, super::Error>;
}
