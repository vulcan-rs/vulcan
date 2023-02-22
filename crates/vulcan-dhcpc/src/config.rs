use std::{
    fs,
    path::PathBuf,
    time::{self, Duration},
};

use serde::Deserialize;
use thiserror::Error;
use toml;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Error while deserializing TOML: {0}")]
    Deserialize(#[from] toml::de::Error),

    #[error("Error while reading TOML config file: {0}")]
    Read(#[from] std::io::Error),
}

#[derive(Deserialize, Default)]
#[serde(default)]
pub struct RawConfig {
    interface: String,
    write_timeout: u64,
    bind_timeout: u64,
    read_timeout: u64,
}

impl RawConfig {
    pub fn from_file(path: PathBuf) -> Result<Self, ConfigError> {
        let b = match fs::read_to_string(path) {
            Ok(b) => b,
            Err(err) => return Err(ConfigError::Read(err)),
        };

        let c: Self = match toml::from_str(&b) {
            Ok(c) => c,
            Err(err) => return Err(ConfigError::Deserialize(err)),
        };

        Ok(c)
    }

    pub fn validate(&self) -> Result<Config, ConfigError> {
        Ok(Config {
            interface: self.interface.clone(),
            write_timeout: Duration::from_secs(self.write_timeout),
            bind_timeout: Duration::from_secs(self.bind_timeout),
            read_timeout: Duration::from_secs(self.read_timeout),
        })
    }
}

pub struct Config {
    pub interface: String,
    pub write_timeout: time::Duration,
    pub bind_timeout: time::Duration,
    pub read_timeout: time::Duration,
}

impl Config {
    pub fn from_file(path: PathBuf) -> Result<Self, ConfigError> {
        let raw_config = RawConfig::from_file(path)?;
        raw_config.validate()
    }
}
