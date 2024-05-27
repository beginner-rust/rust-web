use serde::Deserialize;
use config::{Config, ConfigBuilder, File, Environment};
use anyhow::Result;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub database_url: String,
    pub api_key: String,
    pub log_level: String,
    pub log_file: String,
    // Add more configuration fields as needed
}

pub fn load_config(env: &str) -> Result<Settings> {
    let builder = Config::builder()
        .add_source(File::with_name("config/default"))
        .add_source(File::with_name(&format!("config/{}", env)).required(false))
        .add_source(Environment::with_prefix("APP"));

    let config = builder.build()?;

    config.try_deserialize::<Settings>().map_err(Into::into)
}
