// reference: https://github.com/mehcode/config-rs/blob/master/examples/hierarchical-env/settings.rs
use config::{Config, ConfigError};

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}


pub fn get_config() -> Result<Settings, ConfigError> {
    
    // add config values from a file named config.yaml
    let settings = Config::builder()
    .add_source(config::File::with_name("src/config.yaml"))
    .build()?;
    
    settings.try_deserialize()
}