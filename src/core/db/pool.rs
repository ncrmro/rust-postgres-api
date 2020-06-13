use crate::core::settings;
use anyhow::Result;
use sqlx::{
    error,
    postgres::{PgConnection, PgPool},
    Pool,
};

pub async fn init_db(
    database_config: &settings::Database,
) -> Result<Pool<PgConnection>, error::Error> {
    info!("Initializing database pool");
    let url = &database_config.database_url;
    let pool_result = PgPool::builder().min_size(1).build(url).await;

    match pool_result {
        Ok(pool) => {
            info!("Pool initialized");
            Ok(pool)
        }
        Err(e) => Err(e),
    }
}
