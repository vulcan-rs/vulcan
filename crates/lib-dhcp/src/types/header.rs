use binbuf::prelude::*;

use crate::{constants, types::OpCode};

#[derive(Debug)]
pub struct Header {
    /// Packet op code / message type (1 for BOOTREQUEST and 2 for BOOTREPLY).
    pub opcode: OpCode,

    /// Hardware address type, see ARP section in "Assigned Numbers" RFC.
    pub htype: u8,

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
            htype: constants::HARDWARE_ADDR_TYPE_ETHERNET,
            hlen: constants::HARDWARE_ADDR_LEN_ETHERNET,
            hops: 0,
            xid: 0,
            secs: 0,
            flags: 0,
        }
    }
}

impl Readable for Header {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        let opcode = OpCode::read::<E>(buf)?;
        let [htype, hlen, hops] = u8::read_multi::<E, 3>(buf)?;
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
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        let mut n = 0;

        n += self.opcode.write::<E>(buf)?;
        n += self.htype.write::<E>(buf)?;
        n += self.hlen.write::<E>(buf)?;
        n += self.hops.write::<E>(buf)?;
        n += self.xid.write::<E>(buf)?;
        n += self.secs.write::<E>(buf)?;
        n += self.flags.write::<E>(buf)?;

        Ok(n)
    }
}

impl Header {
    pub fn new() -> Self {
        let mut header = Self::default();

        let xid: u32 = rand::random();
        header.xid = xid;

        header
    }
}
