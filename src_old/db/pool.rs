use crate::settings;
use std::future::Future;
use sqlx::PgPool;

pub fn init_db(database_config: &settings::Database) -> impl Future<Output=PgPool> {
    let db_pool = PgPool::new(database_config.database_url.clone());
    db_pool
}
