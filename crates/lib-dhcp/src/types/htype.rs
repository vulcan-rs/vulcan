use std::fmt::Display;

use binbuf::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum HardwareTypeError {
    #[error("Invalid or unsupported hardware type: {0}")]
    InvalidType(u8),

    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError),
}

#[derive(Debug)]
pub enum HardwareType {
    Ethernet,
}

impl TryFrom<u8> for HardwareType {
    type Error = HardwareTypeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Ethernet),
            _ => Err(HardwareTypeError::InvalidType(value)),
        }
    }
}

impl Display for HardwareType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HardwareType::Ethernet => write!(f, "Ethernet (1)"),
        }
    }
}

impl Readable for HardwareType {
    type Error = HardwareTypeError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        Ok(Self::try_from(buf.pop()?)?)
    }
}

impl Writeable for HardwareType {
    type Error = HardwareTypeError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        // let htype: u8 = (*self).try_into()?;
        // buf.push(htype);
        Ok(1)
    }
}
