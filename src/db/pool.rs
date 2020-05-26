use crate::settings;
use rocket_contrib::databases::diesel::{r2d2, PgConnection};

type ManagedPGConnection = r2d2::ConnectionManager<PgConnection>;
type Pool = r2d2::Pool<ManagedPGConnection>;

pub struct DbConn {
    pub master: r2d2::PooledConnection<ManagedPGConnection>,
}

pub struct DbCollection {
    pub db_conn_pool: Pool,
}

pub fn init_db(database_config: &settings::Database) -> DbCollection {
    let db_manager =
        r2d2::ConnectionManager::<PgConnection>::new(database_config.database_url.clone());
    let db_pool = r2d2::Pool::new(db_manager).expect("Failed to create db pool.");

    DbCollection {
        db_conn_pool: db_pool,
    }
}
