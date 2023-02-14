use binbuf::prelude::*;
use thiserror::Error;

use crate::{
    constants,
    types::{HardwareType, HardwareTypeError, OpCode, OpCodeError},
};

#[derive(Debug, Error)]
pub enum HeaderError {
    #[error("Opcode error: {0}")]
    OpCodeError(#[from] OpCodeError),

    #[error("Hardware type error: {0}")]
    HardwareTypeError(#[from] HardwareTypeError),

    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError),
}

#[derive(Debug)]
pub struct Header {
    /// Packet op code / message type (1 for BOOTREQUEST and 2 for BOOTREPLY).
    pub opcode: OpCode,

    /// Hardware address type, see ARP section in "Assigned Numbers" RFC.
    pub htype: HardwareType,

    /// Hardware address length.
    pub hlen: u8,

    /// Number of hops. Client sets to zero, optionally used by gateways in
    /// cross-gateway booting.
    pub hops: u8,

    /// Transaction ID, a random number, used to match this boot request with
    /// the responses it generates.
    pub xid: u32,

    /// Filled in by client, seconds elapsed since client started trying to
    /// boot.
    pub secs: u16,

    /// Unused
    pub flags: u16,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            opcode: OpCode::BootRequest,
            htype: HardwareType::Ethernet,
            hlen: constants::HARDWARE_ADDR_LEN_ETHERNET,
            hops: 0,
            xid: 0,
            secs: 0,
            flags: 0,
        }
    }
}

impl Readable for Header {
    type Error = HeaderError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let opcode = OpCode::read::<E>(buf)?;
        let htype = HardwareType::read::<E>(buf)?;
        let [hlen, hops] = u8::read_multi::<E, 2>(buf)?;
        let xid = u32::read::<E>(buf)?;
        let secs = u16::read::<E>(buf)?;
        let flags = u16::read::<E>(buf)?;

        Ok(Self {
            opcode,
            htype,
            hlen,
            hops,
            xid,
            secs,
            flags,
        })
    }
}

impl Writeable for Header {
    type Error = HeaderError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let n = bytes_written! {
            self.opcode.write::<E>(buf)?;
            self.htype.write::<E>(buf)?;
            self.hlen.write::<E>(buf)?;
            self.hops.write::<E>(buf)?;
            self.xid.write::<E>(buf)?;
            self.secs.write::<E>(buf)?;
            self.flags.write::<E>(buf)?
        };

        Ok(n)
    }
}

impl Header {
    pub fn new() -> Self {
        let mut header = Self::default();
        header.xid = rand::random();

        header
    }

    pub fn new_with_xid(xid: u32) -> Self {
        let mut header = Self::default();
        header.xid = xid;

        header
    }
}
