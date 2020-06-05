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
    let mut url = &database_config.database_url;
    if database_config.sslmode {
        url = &format!("{}?sslmode=require", url);
    }

    PgPool::new(&database_config.database_url).await
}
