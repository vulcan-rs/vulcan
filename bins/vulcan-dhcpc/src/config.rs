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

pub struct Config {
    pub interface: String,
    pub write_timeout: time::Duration,
    pub bind_timeout: time::Duration,
    pub read_timeout: time::Duration,
}

impl TryFrom<RawConfig> for Config {
    type Error = ConfigError;

    fn try_from(value: RawConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            write_timeout: Duration::from_secs(value.write_timeout),
            bind_timeout: Duration::from_secs(value.bind_timeout),
            read_timeout: Duration::from_secs(value.read_timeout),
            interface: value.interface,
        })
    }
}

impl Config {
    pub fn from_file(path: PathBuf) -> Result<Self, ConfigError> {
        let b = fs::read_to_string(path)?;
        let c: RawConfig = toml::from_str(&b)?;

        Self::try_from(c)
    }
}
