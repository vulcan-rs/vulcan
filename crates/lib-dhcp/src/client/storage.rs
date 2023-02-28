use crate::{types::Lease, IntoLease, Storage, StorageError, StorageResult};

pub struct ClientStorage {
    leases: Vec<Lease>,
}

impl Storage for ClientStorage {
    type Key = usize;

    fn retrieve_lease(&self, key: Self::Key) -> StorageResult<&Lease> {
        match self.leases.get(key) {
            Some(lease) => Ok(lease),
            None => Err(StorageError::RetrieveError),
        }
    }

    fn store_lease<L: IntoLease>(&mut self, key: Self::Key, lease: L) -> StorageResult<()> {
        if key >= self.len() {
            return Err(StorageError::StoreError);
        }

        self.leases.push(lease.into_lease());
        Ok(())
    }

    fn len(&self) -> usize {
        self.leases.len()
    }
}
