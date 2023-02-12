use std::fmt::Display;

use binbuf::prelude::*;

#[derive(Debug)]
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
}
