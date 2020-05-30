use fancy_regex::Regex;
use sqlx::{error, PgConnection, PgPool, Pool};

pub async fn init(settings: planet_express::settings::Settings) -> Pool<PgConnection> {
    let db_pool = planet_express::db::init_db(&settings.database)
        .await
        .unwrap();
    let re = Regex::new(r"(?x)((?<=\d\/)\w*)").unwrap();

    let result = re.captures(&settings.database.database_url);
    let captures = result
        .expect("Error running regex")
        .expect("No match found");

    let db_name = captures.get(1).expect("No group").as_str();

    let test_db_name = format!("{}_test", db_name);

    let test_db =
        sqlx::query(format!("SELECT FROM pg_database WHERE datname = '{}'", test_db_name).as_ref())
            .execute(&db_pool)
            .await
            .unwrap();
    if test_db == 1 {
        sqlx::query(format!("DROP DATABASE {}", test_db_name).as_ref())
            .execute(&db_pool)
            .await
            .unwrap();
    }
    sqlx::query(format!("CREATE DATABASE {}", test_db_name).as_ref())
        .execute(&db_pool)
        .await
        .unwrap();
    db_pool.close().await;

    PgPool::new(format!("{}_test", db_name).as_ref())
        .await
        .unwrap()
}
