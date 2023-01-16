use std::fmt::Display;

use binbuf::prelude::*;

use crate::ProtocolError;

#[derive(Debug)]
pub enum OpCode {
    BootRequest,
    BootReply,
}

impl TryFrom<u8> for OpCode {
    type Error = ProtocolError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::BootRequest),
            2 => Ok(Self::BootReply),
            _ => Err(ProtocolError::InvalidOpCode(value)),
        }
    }
}

impl Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OpCode::BootRequest => write!(f, "BOOTREQUEST"),
            OpCode::BootReply => write!(f, "BOOTREPLY"),
        }
    }
}

impl Readable for OpCode {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        match Self::try_from(buf.pop()?) {
            Ok(opcode) => Ok(opcode),
            Err(err) => Err(BufferError::InvalidData),
        }
    }
}

impl Writeable for OpCode {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        match self {
            OpCode::BootRequest => buf.push(1),
            OpCode::BootReply => buf.push(2),
        }

        Ok(1)
    }
}
