use config::{Config, ConfigError, Environment, File};
use dotenv::dotenv;
use std::env;

#[derive(Deserialize, Clone)]
pub struct Database {
    pub database_url: String,
    pub sslmode: bool,
}

#[derive(Deserialize, Clone)]
pub struct HTTP {
    pub host: String,
    pub port: String,
    pub threads: Option<i32>,
}

#[derive(Deserialize, Clone)]
pub struct AUTH {
    pub jwt_secret: String,
    pub password_salt: String,
}

#[derive(Deserialize, Clone)]
pub struct Settings {
    pub database: Database,
    pub http: HTTP,
    pub auth: AUTH,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        dotenv().ok();
        // HEROKU
        // These env vars are automatically set by Heroku
        // DATABASE_URL is also used by SQLx and is required a compile time to check SQL
        // Just simpler this way...

        match env::var("DATABASE_URL") {
            Ok(database_url) => env::set_var("APP_DATABASE__DATABASE_URL", database_url),
            Err(_e) => (),
        }

        // Same thing heroku mandates using the port at $PORT
        match env::var("PORT") {
            Ok(port) => env::set_var("APP_HTTP__PORT", port),
            Err(_e) => (),
        };
        // END HEROKU

        let mut settings = Config::default();

        // Start off by merging in the "default" configuration file
        settings.merge(File::with_name("config/default").required(false))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("APP_ENV").unwrap_or_else(|_| "development".into());
        info!("Currently running as {:?} environment", env);
        settings.merge(File::with_name(&format!("config/{}", env)).required(false))?;

        // Add in a local configuration file
        // This file shouldn't be checked in to git
        settings.merge(File::with_name("config/local").required(false))?;

        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        settings
            .merge(Environment::with_prefix("APP").separator("__"))
            .unwrap();

        // You can deserialize (and thus freeze) the entire configuration as
        settings.try_into()
    }
}
