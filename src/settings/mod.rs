use config::{Config, ConfigError, Environment, File};
use std::env;

#[derive(Debug, Deserialize)]
pub struct Database {
    pub database_url: String,
}

// #[derive(Debug, Deserialize)]
// pub struct Auth {
//     pub secret: String,
//     pub issuer: String,
//     pub expiry: i64,
// }

#[derive(Debug, Deserialize)]
pub struct HTTP {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database: Database,
    pub http: HTTP,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        // Just simpler this way...
        env::set_var(
            "APP_DATABASE__DATABASE_URL",
            env::var("DATABASE_URL").unwrap(),
        );

        let mut settings = Config::default();

        //         Start off by merging in the "default" configuration file
        settings.merge(File::with_name("config/default"))?;

        // Add in the current environment file
        // Default to 'development' env
        // Note that this file is _optional_
        let env = env::var("APP_ENV").unwrap_or("development".into());
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
