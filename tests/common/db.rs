use anyhow::Result;
use fancy_regex::Regex;
use planet_express::settings::Settings;
use sqlx::{query, Connect, Connection, PgConnection, PgPool, Pool};
use std::fs;

fn test_db_name(settings: &Settings) -> String {
    let re = Regex::new(r"(?x)((?<=\d\/)\w*)").unwrap();

    let result = re.captures(&settings.database.database_url);
    let captures = result
        .expect("Error running regex")
        .expect("No match found");

    let db_name = captures.get(1).expect("No group").as_str();
    let test_db_name = format!("{}_test", db_name);
    test_db_name
}

pub async fn create_testdb(settings: &Settings) -> Result<()> {
    // Take connection from real database and initialize test database
    let mut conn = PgConnection::connect(&settings.database.database_url)
        .await
        .unwrap();
    let db_name = test_db_name(&settings);
    let test_db = query(format!("SELECT FROM pg_database WHERE datname = '{}'", db_name).as_ref())
        .execute(&mut conn)
        .await
        .unwrap();
    if test_db == 0 {
        query(format!("CREATE DATABASE {}", db_name).as_ref())
            .execute(&mut conn)
            .await
            .unwrap();
        let test_url = format!("{}_test", settings.database.database_url);
        // Run migrations on test database
        conn = PgConnection::connect(&test_url).await.unwrap();
        migrations(&mut conn).await;
    }
    conn.close().await.unwrap();
    Ok(())
}

async fn migrations(mut conn: &mut PgConnection) {
    let mut paths: Vec<_> = fs::read_dir("migrations")
        .unwrap()
        .map(|r| r.unwrap())
        .collect();

    query(
        "
create table __migrations
(
    migration varchar(255)                        not null
        constraint __migrations_pkey
            primary key,
    created   timestamp default CURRENT_TIMESTAMP not null
);
        ",
    )
    .execute(&mut conn)
    .await
    .unwrap();
    paths.sort_by_key(|dir| dir.path());

    for path in paths {
        let full_path = format!("{}", path.path().display());
        let contents = fs::read_to_string(full_path).unwrap();

        sqlx::query(contents.as_ref())
            .execute(&mut conn)
            .await
            .unwrap();
        sqlx::query(
            format!(
                "INSERT INTO __migrations (migration) VALUES ('{}');",
                path.path().display()
            )
            .as_ref(),
        )
        .execute(&mut conn)
        .await
        .unwrap();
    }
}

async fn create_schema(mut conn: &mut PgConnection, name: String) -> String {
    sqlx::query(format!("SELECT clone_schema('public','test_{}');", name).as_ref())
        .execute(&mut conn)
        .await
        .unwrap();
    format!("test_{}", name)
}

pub async fn init(settings: Settings, test_name: String) -> Pool<PgConnection> {
    create_testdb(&settings).await.unwrap();

    let test_url = format!("{}_test", settings.database.database_url);
    let mut conn = PgConnection::connect(&test_url).await.unwrap();

    // clone database into schema and return schema connection
    let schema_name = create_schema(&mut conn, test_name).await;
    conn.close();

    PgPool::new(format!("{}?schema={}", &test_url, schema_name).as_ref())
        .await
        .unwrap()
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
