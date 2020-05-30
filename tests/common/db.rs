use fancy_regex::Regex;
use sqlx::{PgConnection, PgPool, Pool};
use std::fs;

async fn migrations(connection: Pool<PgConnection>) {
    let mut paths: Vec<_> = fs::read_dir("migrations")
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    paths.sort_by_key(|dir| dir.path());
    for path in paths {
        let full_path = format!("{}", path.path().display());
        let contents = fs::read_to_string(full_path).unwrap();

        sqlx::query(contents.as_ref())
            .execute(&connection)
            .await
            .unwrap();
    }
}

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

    let test_db_url = format!("{}_test", settings.database.database_url);
    let test_connection = PgPool::new(&test_db_url).await.unwrap();

    migrations(test_connection.clone()).await;
    test_connection
}
