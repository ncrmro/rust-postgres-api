use crate::settings;
use anyhow::Result;
use sqlx::postgres::PgConnection;
use sqlx::postgres::PgPool;
use sqlx::{error, Pool};

pub async fn init_db(
    database_config: &settings::Database,
) -> Result<Pool<PgConnection>, error::Error> {
    PgPool::new(&database_config.database_url).await
}
