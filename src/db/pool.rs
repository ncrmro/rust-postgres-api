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
    let url = &database_config.database_url;
    let ssl = format!("{}?sslmode=require", &url).clone();
    let conn = if database_config.sslmode {
        PgPool::new(&ssl)
    } else {
        PgPool::new(url)
    };
    conn.await
}
