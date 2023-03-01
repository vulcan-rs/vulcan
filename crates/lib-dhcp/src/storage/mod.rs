use std::{fmt::Display, hash::Hash};

use async_trait::async_trait;
use thiserror::Error;

use crate::types::Lease;

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

#[async_trait]
pub trait Storage {
    type Error: Display + std::error::Error + From<StorageError>;
    type Key: Hash + Display;

    async fn retrieve_lease(&self, key: Self::Key) -> Option<Lease>;
    async fn store_lease<L: IntoLease<Error = Self::Error>>(
        &mut self,
        key: Self::Key,
        lease: L,
    ) -> Result<(), Self::Error>;

    async fn run_flush(&self) -> Result<(), Self::Error>;

    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

pub trait IntoLease: Send {
    type Error: std::fmt::Display + std::error::Error;

    fn try_into_lease(&self) -> Result<Lease, Self::Error>;

    fn into_lease(&self) -> Lease {
        match self.try_into_lease() {
            Ok(l) => l,
            Err(err) => panic!("{err}"),
        }
    }
}
