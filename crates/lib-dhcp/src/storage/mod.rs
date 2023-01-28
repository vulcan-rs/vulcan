use std::{
    collections::HashMap,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    net::Ipv4Addr,
    path::PathBuf,
    time::Duration,
};

use tokio::{spawn, task::JoinHandle, time::interval};

mod errors;

pub use errors::*;

trait Storage {
    type Identifier: Display;

    fn save_lease<L, I>(&mut self, ident: I, lease: L)
    where
        L: IntoLease,
        I: Into<Self::Identifier>;
    fn get_lease<I>(&self, ident: I) -> Result<&Lease, StorageError>
    where
        I: Into<Self::Identifier>;

    fn is_empty(&self) -> bool;
    fn clear(&mut self);
}

trait IntoLease: From<String> {
    // type Error: Display + Error;

    fn into_lease(&self) -> Lease;
}

#[derive(Debug, Default)]
pub struct FileStorage {
    leases: HashMap<StorageIdentifier, Lease>,
    flush_interval: Duration,
    pending_changes: bool,
    file_path: PathBuf,
}

impl Storage for FileStorage {
    type Identifier = StorageIdentifier;

    fn save_lease<L, I>(&mut self, ident: I, lease: L)
    where
        L: IntoLease,
        I: Into<Self::Identifier>,
    {
        let ident: Self::Identifier = ident.into();
        let lease = lease.into_lease();

        // Check if lease already exists
        if let Some(existing_lease) = self.leases.get(&ident) {
            // If there are the same, just do nothing and return
            if *existing_lease == lease {
                return;
            }
        }

        self.leases.insert(ident, lease);
        self.pending_changes = true;
    }

    fn get_lease<I>(&self, ident: I) -> Result<&Lease, StorageError>
    where
        I: Into<Self::Identifier>,
    {
        match self.leases.get(&ident.into()) {
            Some(lease) => Ok(lease),
            None => Err(StorageError::NoSuchLease),
        }
    }

    fn is_empty(&self) -> bool {
        self.leases.is_empty()
    }

    fn clear(&mut self) {
        self.leases.clear()
    }
}

impl FileStorage {
    pub fn new(file_path: PathBuf, flush_interval: Duration) -> Self {
        Self {
            flush_interval,
            file_path,
            ..Default::default()
        }
    }

    pub fn load_leases(&mut self) -> Result<usize, StorageError> {
        let file: File = File::open(&self.file_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let (ident, lease) = match line.split_once(' ') {
                Some(tuple) => tuple,
                None => return Err(StorageError::MalformedLease),
            };

            let ident = StorageIdentifier::try_from(ident)?;
            let lease = Lease::try_from(lease)?;

            self.leases.insert(ident, lease);
        }

        Ok(self.leases.len())
    }

    pub fn run_flush_ticker(&self) -> JoinHandle<()> {
        let mut interval = interval(self.flush_interval);
        let path = self.file_path.clone();

        let handle = spawn(async move {
            loop {
                interval.tick().await;
                println!("Flush")
            }
        });

        handle
    }
}

#[derive(Debug, Default, Eq, Hash, PartialEq)]
pub struct StorageIdentifier {
    mac_addr: String,
    hostname: String,
}

impl Display for StorageIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.hostname, self.mac_addr)
    }
}

impl TryFrom<&str> for StorageIdentifier {
    type Error = StorageIdentifierError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (mac_addr, hostname) = match value.split_once('-') {
            Some(tuple) => tuple,
            None => {
                return Err(StorageIdentifierError(
                    "Expected an identifier with the form {mac_address}-{hostname}".to_string(),
                ))
            }
        };

        Ok(Self {
            mac_addr: mac_addr.into(),
            hostname: hostname.into(),
        })
    }
}

impl StorageIdentifier {
    pub fn new<T>(mac_addr: T, hostname: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            mac_addr: mac_addr.into(),
            hostname: hostname.into(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Lease {
    ip_addr: Ipv4Addr,
    mac_addr: String,
    hostname: String,
    renewal_time: u32,
    rebinding_time: u32,
}

impl TryFrom<&str> for Lease {
    type Error = LeaseError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(' ').collect();

        if parts.len() != 5 {
            return Err(LeaseError::InvalidPartsCount(parts.len()));
        }

        Ok(Self {
            ip_addr: parts[0].parse()?,
            mac_addr: parts[1].into(),
            hostname: parts[2].into(),
            renewal_time: parts[3].parse()?,
            rebinding_time: parts[4].parse()?,
        })
    }
}

// impl Display for Lease {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{} {} {} {} {}",
//             self.ip_addr, self.mac_addr, self.hostname, self.renewal_time, self.rebinding_time
//         )
//     }
// }
