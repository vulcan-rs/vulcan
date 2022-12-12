use binbuf::prelude::*;

#[derive(Debug)]
pub struct Addrs {
    /// Client IP address; filled in by client in BOOTREQUEST if known.
    pub ciaddr: u32,

    /// 'Your' (client) IP address. Filled by server if client doesn't know
    /// its own address (ciaddr was 0).
    pub yiaddr: u32,

    /// Server IP address. Returned in BOOTREPLY by server.
    pub siaddr: u32,

    /// Gateway IP address, used in optional cross-gateway booting.
    pub giaddr: u32,

    /// Client hardware address, filled in by client (16 octets).
    pub chaddr: u128,
}

impl Default for Addrs {
    fn default() -> Self {
        Self {
            ciaddr: 0,
            yiaddr: 0,
            siaddr: 0,
            giaddr: 0,
            chaddr: 0,
        }
    }
}

impl Readable for Addrs {
    const SUPPORTED_ENDIANNESS: SupportedEndianness = SupportedEndianness::BigEndian;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> ReadBufferResult<Self> {
        Self::supports::<E>()?;

        let [ciaddr, yiaddr, siaddr, giaddr] = u32::read_multi_be(buf)?;
        let chaddr = u128::read_be(buf)?;

        Ok(Self {
            ciaddr,
            yiaddr,
            siaddr,
            giaddr,
            chaddr,
        })
    }
}

impl Writeable for Addrs {
    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> WriteBufferResult {
        let mut n = 0;

        n += self.ciaddr.write_be(buf)?;
        n += self.yiaddr.write_be(buf)?;
        n += self.siaddr.write_be(buf)?;
        n += self.giaddr.write_be(buf)?;
        n += self.chaddr.write_be(buf)?;

        Ok(n)
    }
}
