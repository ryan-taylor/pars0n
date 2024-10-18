use std::fs;
use anyhow::Result;
use serde::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Config {
    pub folders: Folders,
}

#[derive(Deserialize)]
pub struct Folders {
    pub input_folder: String,
    // Removed output_folder
}

pub fn read_config() -> Result<Config> {
    let config_content = fs::read_to_string("config.toml")?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}
