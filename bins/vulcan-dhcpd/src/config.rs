use std::{fs, path::PathBuf};

use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Error while reading TOML config file: {0}")]
    Read(#[from] std::io::Error),

    #[error("Error while deserializing TOML: {0}")]
    Deserialize(#[from] toml::de::Error),
}

#[derive(Debug, Deserialize)]
pub struct RawConfig {
    pub storage: RawStorageOptions,
    pub server: RawServerOptions,
    pub rebind_time: u32,
    pub renew_time: u32,
}

#[derive(Debug, Deserialize)]
pub struct RawStorageOptions {
    #[serde(rename = "type")]
    ty: StorageType,
    path: PathBuf,
}

#[derive(Debug)]
pub struct StorageOptions {
    ty: StorageType,
    path: PathBuf,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StorageType {
    File,
}

#[derive(Debug, Deserialize)]
pub struct RawServerOptions {
    interface: String,
    write_timeout: u64,
    bind_timeout: u64,
    read_timeout: u64,
}

#[derive(Debug)]
pub struct ServerOptions {
    interface: String,
    write_timeout: u64,
    bind_timeout: u64,
    read_timeout: u64,
}

#[derive(Debug)]
pub struct Config {
    pub storage: StorageOptions,
    pub server: ServerOptions,
    pub rebind_time: u32,
    pub renew_time: u32,
}

impl TryFrom<RawConfig> for Config {
    type Error = ConfigError;

    fn try_from(value: RawConfig) -> Result<Self, Self::Error> {
        Ok(Self {
            storage: StorageOptions {
                ty: value.storage.ty,
                path: value.storage.path,
            },
            server: ServerOptions {
                interface: value.server.interface,
                write_timeout: value.server.write_timeout,
                bind_timeout: value.server.bind_timeout,
                read_timeout: value.server.read_timeout,
            },
            rebind_time: value.rebind_time,
            renew_time: value.renew_time,
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
