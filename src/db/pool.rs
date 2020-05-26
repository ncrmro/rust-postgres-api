use crate::settings;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgConnection;
use sqlx::{error, Pool};
use anyhow::Result;


pub async fn init_db(database_config: &settings::Database) -> Result<Pool<PgConnection>, error::Error> {
    PgPool::new(&database_config.database_url).await
}
