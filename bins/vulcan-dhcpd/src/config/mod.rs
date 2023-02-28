use std::{fs, path::PathBuf};

use serde::Deserialize;
use thiserror::Error;

use crate::constants::DEFAULT_CONFIG_FILE_PATH;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Error while reading TOML config file: {0}")]
    Read(#[from] std::io::Error),

    #[error("Error while deserializing TOML: {0}")]
    Deserialize(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub(crate) renewal_time: u32,
    pub(crate) rebinding_time: u32,
    pub(crate) storage: StorageOptions,
}

#[derive(Debug, Deserialize)]
pub struct StorageOptions {
    #[serde(rename = "type")]
    ty: StorageType,
    path: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageType {
    File,
}

impl Config {
    pub fn read(path: Option<PathBuf>) -> Result<Self, ConfigError> {
        let mut path = path;

        if path.is_none() {
            path = Some(PathBuf::from(DEFAULT_CONFIG_FILE_PATH));
        }

        let s = match fs::read_to_string(path.unwrap()) {
            Ok(s) => s,
            Err(err) => return Err(ConfigError::Read(err)),
        };

        let c: Self = match toml::from_str(&s) {
            Ok(c) => c,
            Err(err) => return Err(ConfigError::Deserialize(err)),
        };

        Ok(c)
    }
}
