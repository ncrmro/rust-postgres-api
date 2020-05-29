use crate::settings;
use anyhow::Result;
use sqlx::{
    error,
    postgres::{PgConnection, PgPool},
    Pool,
};

pub async fn init_db(
    database_config: &settings::Database,
) -> Result<Pool<PgConnection>, error::Error> {
    PgPool::new(&database_config.database_url).await
}
