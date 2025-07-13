use std::path::Path;

use anyhow::{Context, Result};

pub use crate::structs::{AuthConfig, PokeConfigLoader};

mod structs;

pub fn load_from_file<Config: serde::de::DeserializeOwned>(path: &str) -> Result<Config> {
    let content = std::fs::read(path).context("Failed to read config file")?;

    serde_json::from_slice(&content).context("Failed to parse config file")
}

pub fn write_auth_config<P: AsRef<Path>>(path: P, config: &AuthConfig) -> Result<()> {
    let file = std::fs::File::create(path).context("Failed to create config file")?;

    serde_json::to_writer(file, config).context("Failed to write config file")
}

pub fn write_config_file<P: AsRef<Path>>(path: P, config: &PokeConfigLoader) -> Result<()> {
    let file = std::fs::File::create(path).context("Failed to create config file")?;

    serde_json::to_writer_pretty(file, config).context("Failed to write config file")
}
