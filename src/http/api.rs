use crate::db::init_db;
use crate::settings::Settings;
use rocket;
use rocket::config::{Config, Environment};

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/auth")]
fn authenticate() -> &'static str {
    "Authenticate"
}

pub fn main(settings: Settings) {
    rocket_init(settings).launch();
}

pub fn rocket_init(settings: Settings) -> rocket::Rocket {
    let dbs = init_db(settings);
    let config = Config::build(Environment::Development)
        .address("0.0.0.0")
        .extra("databases", dbs)
        .finalize()
        .unwrap();

    rocket::custom(config).mount("/", routes![hello, authenticate])
}
