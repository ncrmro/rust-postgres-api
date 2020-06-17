use fancy_regex::Regex;
use futures::executor::block_on;
use src::core::db::{query, Connect, Connection, PgConnection, PgPool, Pool};
use src::core::settings::Settings;
use std::process::Command;
use std::sync::Once;

static START: Once = Once::new();

fn db_name(settings: &Settings) -> String {
    let re = Regex::new(r"(?x)((?<=\d\/)\w*)").unwrap();

    let result = re.captures(&settings.database.database_url);
    let captures = result
        .expect("Error running regex")
        .expect("No match found");

    let db_name = captures.get(1).expect("No group").as_str();
    // let test_db_name = format!("{}_test", db_name);
    String::from(db_name)
}

fn get_test_db_name(settings: &Settings) -> String {
    format!("{}_test", db_name(settings))
}

pub async fn create_testdb(settings: &Settings) {
    // Take connection from real database and initialize test database
    let mut conn = PgConnection::connect(&settings.database.database_url)
        .await
        .unwrap();
    let test_db_name = get_test_db_name(&settings);
    let test_db =
        query(format!("SELECT FROM pg_database WHERE datname = '{}'", test_db_name).as_ref())
            .execute(&mut conn)
            .await
            .unwrap();
    if test_db == 0 {
        query(format!("CREATE DATABASE {}", test_db_name).as_ref())
            .execute(&mut conn)
            .await
            .unwrap();
        let test_url = format!("{}_test", settings.database.database_url);
        // Run migrations on test database
        conn = PgConnection::connect(&test_url).await.unwrap();
        migrations(settings).await;
    }
    conn.close().await.unwrap();
}

async fn migrations(settings: &Settings) {
    // Migrations are organized this way to allow multiple statements in a file
    let test_url = format!("{}_test", settings.database.database_url);
    Command::new("sqlx")
        .env("DATABASE_URL", test_url)
        .arg("migrate")
        .arg("run")
        .spawn()
        .unwrap();
    // Give the database a time to catch up
    let ten_millis = std::time::Duration::from_secs(1);
    std::thread::sleep(ten_millis);
}

async fn create_schema(mut conn: &mut PgConnection, name: String) -> String {
    sqlx::query(format!("SELECT clone_schema('public','test_{}');", name).as_ref())
        .execute(&mut conn)
        .await
        .unwrap();
    format!("test_{}", name)
}

pub async fn init(settings: Settings, test_name: String) -> Pool<PgConnection> {
    START.call_once(|| {
        block_on(create_testdb(&settings));
    });

    let test_url = format!("{}_test", settings.database.database_url);
    let mut conn = PgConnection::connect(&test_url).await.unwrap();

    // clone database into schema and return schema connection
    let schema_name = create_schema(&mut conn, test_name).await;
    conn.close();

    let conn = PgPool::new(format!("{}?schema={}", &test_url, schema_name).as_ref())
        .await
        .unwrap();

    sqlx::query(format!("SET search_path to {};", schema_name).as_ref())
        .execute(&conn)
        .await
        .unwrap();
    conn
}

pub async fn down(settings: Settings, test_id: String) {
    let test_url = format!("{}_test", settings.database.database_url);
    let mut conn = PgConnection::connect(test_url).await.unwrap();
    query(format!("DROP SCHEMA test_{} CASCADE", test_id).as_ref())
        .execute(&mut conn)
        .await
        .unwrap();
    conn.close();
}
