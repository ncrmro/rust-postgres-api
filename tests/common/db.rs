use anyhow::Result;
use fancy_regex::Regex;
use planet_express::settings::Settings;
use sqlx::{query, Connect, Connection, Executor, PgConnection};
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

async fn init_testdb(settings: &Settings) -> Result<()> {
    // Take connection from real database and initialize test database
    let mut conn = PgConnection::connect(&settings.database.database_url)
        .await
        .unwrap();
    let db_name = test_db_name(&settings);
    query(format!("DROP DATABASE IF EXISTS {}", db_name).as_ref())
        .execute(&mut conn)
        .await
        .unwrap();
    let test_db = query(format!("SELECT FROM pg_database WHERE datname = '{}'", db_name).as_ref())
        .execute(&mut conn)
        .await
        .unwrap();
    if test_db == 0 {
        query(format!("CREATE DATABASE {}", db_name).as_ref())
            .execute(&mut conn)
            .await
            .unwrap();
    }
    Ok(conn.close().await.unwrap())
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

pub async fn init(settings: Settings) -> PgConnection {
    init_testdb(&settings).await;

    let mut conn = PgConnection::connect(&format!("{}_test", settings.database.database_url))
        .await
        .unwrap();
    migrations(&mut conn).await;
    conn
}
