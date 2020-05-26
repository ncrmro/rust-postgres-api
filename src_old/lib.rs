#![feature(proc_macro_hygiene, decl_macro, plugin)]

extern crate chrono;
extern crate config;
extern crate serde;
#[macro_use]
extern crate serde_derive;
// #[macro_use]
extern crate serde_json;

// pub mod db;
// pub mod http;
pub mod settings;

// use settings::Settings;

// pub async fn init() -> std::result::Result<(), std::io::Error> {
//     let settings = Settings::new().unwrap();
//     http::api::main(settings).await
// }
