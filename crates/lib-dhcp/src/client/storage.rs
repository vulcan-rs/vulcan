use async_trait::async_trait;

use crate::{types::Lease, IntoLease, Storage, StorageError};

#[derive(Debug, Default)]
pub struct ClientStorage {
    leases: Vec<Lease>,
}

// #[async_trait]
// impl Storage for ClientStorage {
//     type Error = StorageError;
//     type Key = usize;

//     async fn retrieve_lease(&self, key: Self::Key) -> Result<&Lease, Self::Error> {
//         match self.leases.get(key) {
//             Some(lease) => Ok(lease),
//             None => Err(StorageError::RetrieveError),
//         }
//     }

//     async fn store_lease<L: IntoLease>(
//         &mut self,
//         key: Self::Key,
//         lease: L,
//     ) -> Result<(), Self::Error> {
//         if key >= self.len() {
//             return Err(StorageError::StoreError);
//         }

//         self.leases.push(lease.into_lease());
//         Ok(())
//     }

//     fn len(&self) -> usize {
//         self.leases.len()
//     }
// }

impl ClientStorage {
    pub fn new() -> Self {
        ClientStorage::default()
    }
}
