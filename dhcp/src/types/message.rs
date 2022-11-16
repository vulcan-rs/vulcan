use std::fmt::Display;

use binbuf::{
    ReadBuffer, ReadBufferResult, Readable, ReadableMulti, ToReadBuffer, WriteBuffer,
    WriteBufferResult, Writeable,
};

use crate::constants;

/// [`Message`] describes a complete BOOTP message. The same packet field
/// layout is used in both directions.
///
/// ### See
///
/// RFC 951 - Section 3 - Packet Format: https://datatracker.ietf.org/doc/html/rfc951#section-3
pub struct Message {
    /// Packet op code / message type (1 for BOOTREQUEST and 2 for BOOTREPLY).
    pub opcode: u8,

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

    /// Optional server host name, null terminated string (64 octets).
    pub sname: Vec<u8>,

    /// Boot file name, null terminated string. 'Generic' name or null in
    /// BOOTREQUEST, fully qualified directory-path name in bootreply
    /// (128 octets).
    pub file: Vec<u8>,

    /// Optional vendor-specific area, e.g. could be hardware type/serial on
    /// request, or 'capability' / remote file system handle on reply. This
    /// info may be set aside for use by a third phase bootstrap or kernel
    /// (64 octets).
    pub vend: Vec<u8>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "MSG TY: {:02x?}; HW TY: {:02x?}; HW ADDR LEN: {:02x?}; HOPS: {:02x?}; ID: {:02x?}; SECS: {:02x?}; FLAGS: {:02x?}\n\
            ; Client IP: {:08x?}\n\
            ; Your (client) IP: {:08x?}\n\
            ; Next server IP: {:08x?}\n\
            ; Relay agent IP: {:08x?}\n\
            ; Client MAC addr: {:016x?}\n\
            ; Server host name: {:02x?}",
            self.opcode, self.htype, self.hlen, self.hops, self.xid, self.secs, self.flags,
            self.ciaddr,
            self.yiaddr,
            self.siaddr,
            self.giaddr,
            self.chaddr,
            self.sname
        )
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            opcode: constants::BOOTP_OPCODE_REQUEST,
            htype: constants::HARDWARE_ADDR_TYPE_ETHERNET,
            hlen: constants::HARDWARE_ADDR_LEN_ETHERNET,
            hops: 0,
            xid: 0,
            secs: 0,
            flags: 0,
            ciaddr: 0,
            yiaddr: 0,
            siaddr: 0,
            giaddr: 0,
            chaddr: 0,
            sname: vec![0; 64],
            file: vec![0; 128],
            vend: vec![0; 64],
        }
    }
}

impl Readable for Message {
    fn read(buf: &mut ReadBuffer) -> ReadBufferResult<Self> {
        let [opcode, htype, hlen, hops] = u8::read_multi(buf)?;
        let xid = u32::read(buf)?;
        let secs = u16::read(buf)?;
        let flags = u16::read(buf)?;
        let [ciaddr, yiaddr, siaddr, giaddr] = u32::read_multi(buf)?;
        let chaddr = u128::read(buf)?;
        let sname = buf.read_vec(64)?;
        let file = buf.read_vec(128)?;
        let vend = buf.read_vec(64)?;

        Ok(Self {
            opcode,
            htype,
            hlen,
            hops,
            xid,
            secs,
            flags,
            ciaddr,
            yiaddr,
            siaddr,
            giaddr,
            chaddr,
            sname,
            file,
            vend,
        })
    }
}

impl Writeable for Message {
    fn write(&self, buf: &mut WriteBuffer) -> WriteBufferResult {
        self.opcode.write(buf)?;
        self.htype.write(buf)?;
        self.hlen.write(buf)?;
        self.hops.write(buf)?;
        self.xid.write(buf)?;
        self.secs.write(buf)?;
        self.flags.write(buf)?;
        self.ciaddr.write(buf)?;
        self.yiaddr.write(buf)?;
        self.siaddr.write(buf)?;
        self.giaddr.write(buf)?;
        self.chaddr.write(buf)?;
        self.sname.write(buf)?;
        self.file.write(buf)?;
        self.vend.write(buf)
    }
}

impl Message {
    /// Create a new BOOTP [`Message`]. This automatically generates a random
    /// transaction ID (xid). The remaining fields are filled with the default
    /// values specified by the [`Default`] implementation.
    pub fn new(secs: u16, ciaddr: u32, chaddr: u128) -> Self {
        let xid: u32 = rand::random();

        Self {
            xid,
            secs,
            ciaddr,
            chaddr,
            ..Default::default()
        }
    }
}
