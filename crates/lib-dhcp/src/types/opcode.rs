use std::fmt::Display;

use binbuf::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OpCodeError {
    #[error("Invalid opcode - expected '1' or '2', got '{0}'")]
    InvalidCode(u8),

    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError),
}

#[derive(Debug)]
pub enum OpCode {
    BootRequest,
    BootReply,
}

impl TryFrom<u8> for OpCode {
    type Error = OpCodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::BootRequest),
            2 => Ok(Self::BootReply),
            _ => Err(OpCodeError::InvalidCode(value)),
        }
    }
}

impl From<OpCode> for u8 {
    fn from(value: OpCode) -> Self {
        match value {
            OpCode::BootRequest => 1,
            OpCode::BootReply => 2,
        }
    }
}

impl From<&OpCode> for u8 {
    fn from(value: &OpCode) -> Self {
        match value {
            OpCode::BootRequest => 1,
            OpCode::BootReply => 2,
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::BootRequest => write!(f, "Boot Request (1)"),
            OpCode::BootReply => write!(f, "Boot Reply (2)"),
        }
    }
}

impl Readable for OpCode {
    type Error = OpCodeError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        Self::try_from(buf.pop()?)
    }
}

impl Writeable for OpCode {
    type Error = OpCodeError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        buf.push(u8::from(self));
        Ok(1)
    }
}
