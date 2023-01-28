use std::{error::Error, fmt::Display, net::AddrParseError, num::ParseIntError};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("No such lease")]
    NoSuchLease,

    #[error("Lease identifier error: {0}")]
    LeaseIdentifierError(#[from] StorageIdentifierError),

    #[error("Lease error: {0}")]
    LeaseError(#[from] LeaseError),

    #[error("Malformed lease")]
    MalformedLease,

    #[error("Save lease error")]
    SaveLeaseError,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct StorageIdentifierError(pub String);

impl Display for StorageIdentifierError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Error for StorageIdentifierError {}

#[derive(Debug, Error)]
pub enum LeaseError {
    #[error("IP address parse error: {0}")]
    AddrParseError(#[from] AddrParseError),

    #[error("Integer parse error: {0}")]
    ParseIntError(#[from] ParseIntError),

    #[error("Invalid parts count - expected 5, got {0}")]
    InvalidPartsCount(usize),
}
