use std::{
    collections::HashMap,
    fmt::Display,
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

use async_trait::async_trait;
use serde_json;
use thiserror::Error;
use tokio::{
    self,
    fs::File,
    io::{AsyncWriteExt, BufWriter},
    task::JoinError,
    time,
};

use crate::{
    types::{HardwareAddr, Lease},
    IntoLease, Storage, StorageError,
};

pub struct ServerStorage {
    leases: Arc<Mutex<HashMap<String, Lease>>>,

    leases_file_path: PathBuf,
    flush_interval: u64,
    changed: bool,
}

#[derive(Debug, Hash)]
pub struct StorageKey {
    hardware_addr: HardwareAddr,
    hostname: Option<String>,
}

impl Display for StorageKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.hostname {
            Some(_) => write!(
                f,
                "{}_{}",
                self.hostname.as_ref().unwrap(),
                self.hardware_addr
            ),
            None => write!(f, "{}", self.hardware_addr),
        }
    }
}

#[derive(Debug, Error)]
pub enum ServerStorageError {
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("join error: {0}")]
    JoinError(#[from] JoinError),

    #[error("failed to deserialize/serialize from/into JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("storage error: {0}")]
    StorageError(#[from] StorageError),
}

#[async_trait]
impl Storage for ServerStorage {
    type Error = ServerStorageError;
    type Key = StorageKey;

    async fn retrieve_lease(&self, key: Self::Key) -> Option<Lease> {
        let key = key.to_string();
        let leases = self.leases.lock().unwrap();

        // leases.get(&key)
        None
    }

    async fn store_lease<L: IntoLease>(
        &mut self,
        key: Self::Key,
        lease: L,
    ) -> Result<(), Self::Error> {
        // self.changed = true;

        let lease = lease.into_lease();
        let key = key.to_string();

        let mut leases = self.leases.lock().unwrap();
        leases.insert(key, lease);

        Ok(())
    }

    async fn run_flush(&self) -> Result<(), Self::Error> {
        let leases_file_path = self.leases_file_path.clone();
        let leases = self.leases.clone();

        let interval = self.flush_interval;
        let changed = self.changed;

        tokio::spawn(
            async move { handle_flush(interval, changed, leases_file_path, leases).await },
        );

        Ok(())
    }

    fn len(&self) -> usize {
        let guard = self.leases.lock().unwrap();
        guard.len()
    }
}

impl ServerStorage {
    pub fn new(leases_file_path: PathBuf, flush_interval: u64) -> Self {
        Self {
            leases: Arc::new(Mutex::new(HashMap::new())),
            changed: false,
            leases_file_path,
            flush_interval,
        }
    }
}

async fn handle_flush(
    flush_interval: u64,
    changed: bool,
    leases_file_path: PathBuf,
    leases: Arc<Mutex<HashMap<String, Lease>>>,
) -> Result<(), ServerStorageError> {
    let mut interval = time::interval(Duration::from_secs(flush_interval));
    interval.tick().await;

    loop {
        // Await next interval tick
        interval.tick().await;

        // Check if there are any new leases added since we last flushed.
        // If not, we skip flushing and wait for the next interval tick.
        if !changed {
            continue;
        }

        // Open the leases file
        // FIXME (Techassi): This will overwrite the file everytime. We
        // should diff here to only write the changes.
        let leases_file = File::create(leases_file_path.clone()).await?;

        // Create a buffered writer on the file to write lease by lease
        let mut writer = BufWriter::new(leases_file);

        // Serialize list of leases into JSON string
        let mut output = String::new();
        {
            let guard = leases.lock().unwrap();
            output = serde_json::to_string_pretty(&*guard)?;
        }

        // Write JSON string to file using the buffered writer
        writer.write(output.as_bytes()).await?;
        writer.flush().await?
    }
}
