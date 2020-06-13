use crate::settings;
use sqlx::PgPool;
use std::future::Future;

pub fn init_db(database_config: &settings::Database) -> impl Future<Output = PgPool> {
    let db_pool = PgPool::new(database_config.database_url.clone());
    db_pool
}
