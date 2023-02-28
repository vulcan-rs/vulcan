use std::hash::Hash;

use thiserror::Error;

use crate::types::Lease;

pub type StorageResult<T> = Result<T, StorageError>;

#[derive(Debug, Error)]
pub enum StorageError {
    /// This error indicates that the storage provider failed to retrieve the
    /// requested lease.
    #[error("failed to retrieve DHCP lease")]
    RetrieveError,

    /// This error indicates that the storage provider failed to save the
    /// provided lease.
    #[error("failed to store DHCP lease")]
    StoreError,

    /// This indicates some other unknown error occured.
    #[error("storage error: {0}")]
    Unknown(String),
}

pub trait Storage {
    type Key: Hash;

    fn retrieve_lease(&self, key: Self::Key) -> StorageResult<&Lease>;
    fn store_lease<L: IntoLease>(&mut self, key: Self::Key, lease: L) -> StorageResult<()>;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait IntoLease {
    type Error: std::fmt::Display + std::error::Error;

    fn try_into_lease(&self) -> Result<Lease, Self::Error>;

    fn into_lease(&self) -> Lease {
        match self.try_into_lease() {
            Ok(l) => l,
            Err(err) => panic!("{err}"),
        }
    }
}
