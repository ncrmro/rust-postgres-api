#![feature(proc_macro_hygiene, decl_macro, plugin)]
// #![plugin(rocket_codegen)]

extern crate config;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod db;
pub mod http;
pub mod settings;

use settings::Settings;

pub fn init() {
    let settings = Settings::new().unwrap();
    http::api::main(settings);
}
