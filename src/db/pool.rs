use crate::settings::Settings;
use rocket::config::Value;
use std::collections::HashMap;

pub fn init_db(settings: Settings) -> HashMap<&'static str, rocket::config::Value> {
    let mut db_config = HashMap::new();
    let mut dbs = HashMap::new();

    db_config.insert("url", settings.database.database_url);
    dbs.insert("my_db", Value::from(db_config));
    return dbs;
}
