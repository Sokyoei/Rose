use config::{Config, ConfigError};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Database {
    pub url: String,
}

#[derive(Deserialize)]
pub struct App {
    pub host: String,
    pub port: u16,
}

#[derive(Deserialize)]
pub struct Settings {
    pub debug: bool,
    pub database: Database,
    pub app: App,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let settings = Config::builder()
            .add_source(config::File::with_name("config"))
            .build()?;

        settings.try_deserialize()
    }
}
