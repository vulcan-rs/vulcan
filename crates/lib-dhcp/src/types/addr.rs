use std::{fmt::Display, num::ParseIntError};

use binbuf::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseHardwareAddrError {
    #[error("Invalid byte: {0}")]
    InvalidByte(#[from] ParseIntError),

    #[error("Invalid separator, expected ':'")]
    InvalidSeparator,

    #[error("Invalid length - expected < 16, got {0}")]
    InvalidLength(usize),
}

#[derive(Debug, Clone, Hash, Deserialize, Serialize)]
pub struct HardwareAddr {
    padding: Vec<u8>,
    addr: Vec<u8>,
}

impl Display for HardwareAddr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, Padding: {:?}", self.addr, self.padding)
    }
}

impl Default for HardwareAddr {
    fn default() -> Self {
        Self {
            padding: vec![0; 16],
            addr: vec![],
        }
    }
}

impl Writeable for HardwareAddr {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        if self.addr.len() + self.padding.len() != 16 {
            return Err(BufferError::InvalidData);
        }

        let n = bytes_written! {
            self.addr.write::<E>(buf)?;
            self.padding.write::<E>(buf)?
        };

        Ok(n)
    }
}

impl TryFrom<String> for HardwareAddr {
    type Error = ParseHardwareAddrError;

    fn try_from(input: String) -> Result<Self, Self::Error> {
        if !input.contains(':') {
            return Err(ParseHardwareAddrError::InvalidSeparator);
        }

        let input = input.trim();
        let bytes: Vec<_> = input.split(':').collect();

        if bytes.len() > 16 {
            return Err(ParseHardwareAddrError::InvalidLength(bytes.len()));
        }

        let mut addr: Vec<u8> = Vec::new();

        for byte in bytes {
            addr.push(u8::from_str_radix(byte, 16)?);
        }

        Ok(Self {
            padding: vec![0; 16 - addr.len()],
            addr,
        })
    }
}

impl TryFrom<&String> for HardwareAddr {
    type Error = <Self as TryFrom<String>>::Error;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        Self::try_from(value.clone())
    }
}

impl HardwareAddr {
    pub fn read<E: Endianness>(buf: &mut ReadBuffer, hlen: u8) -> Result<Self, BufferError> {
        // The client hardware address can only be 16 bytes long at max
        if hlen > 16 {
            return Err(BufferError::InvalidData);
        }

        // Read full length hardware address and then split at hlen, the rest
        // is padding
        let full_addr = buf.read_vec(16)?;
        let (addr, padding) = full_addr.split_at(hlen as usize);

        Ok(Self {
            padding: padding.to_vec(),
            addr: addr.to_vec(),
        })
    }

    pub fn len(&self) -> usize {
        self.addr.len()
    }

    pub fn is_empty(&self) -> bool {
        self.addr.is_empty()
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        self.addr.to_owned()
    }
}

#[test]
fn test_hardware_address_from_string() {
    let addr = String::from("DE:AD:BE:EF:12:34");
    match HardwareAddr::try_from(addr) {
        Ok(addr) => {
            assert_eq!(addr.addr, vec![222, 173, 190, 239, 18, 52]);
            assert_eq!(addr.padding.len(), 10);
        }
        Err(err) => panic!("{}", err),
    };
}
