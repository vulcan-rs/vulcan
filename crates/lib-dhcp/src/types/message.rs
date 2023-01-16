use std::fmt::Display;

use binbuf::prelude::*;

use crate::{
    constants,
    types::{Addrs, Header},
};

/// [`Message`] describes a complete DHCP message. The same packet field
/// layout is used in both directions.
///
/// ### See
///
/// RFC 2131 - Section 2 - Protocol Summary: https://datatracker.ietf.org/doc/html/rfc2131#section-2
#[derive(Debug)]
pub struct Message {
    /// Header fields like the opcode, transaction id and additional flags.
    header: Header,

    /// Different IP and hardware addresses.
    addrs: Addrs,

    /// Optional server host name, null terminated string (64 octets).
    sname: Vec<u8>,

    /// Boot file name, null terminated string. 'Generic' name or null in
    /// BOOTREQUEST, fully qualified directory-path name in bootreply
    /// (128 octets).
    file: Vec<u8>,

    /// Originally this was called 'vendor extensions' in the BOOTP RFC. The
    /// RFC states:
    ///
    /// Optional vendor-specific area, e.g. could be hardware type/serial on
    /// request, or 'capability' / remote file system handle on reply. This
    /// info may be set aside for use by a third phase bootstrap or kernel
    /// (64 octets).
    ///
    /// The DHCP RFC renames this filed to 'options'.
    options: Vec<u8>,
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
            self.header.opcode, self.header.htype, self.header.hlen, self.header.hops, self.header.xid, self.header.secs, self.header.flags,
            self.addrs.ciaddr,
            self.addrs.yiaddr,
            self.addrs.siaddr,
            self.addrs.giaddr,
            self.addrs.chaddr,
            self.sname
        )
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            header: Header::default(),
            addrs: Addrs::default(),
            sname: vec![0; 64],
            file: vec![0; 128],
            options: vec![0; 64],
        }
    }
}

impl Readable for Message {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        let header = Header::read::<E>(buf)?;
        let addrs = Addrs::read::<E>(buf)?;
        let sname = buf.read_vec(64)?;
        let file = buf.read_vec(128)?;

        match buf.peekn::<4>() {
            Some(m) if m == constants::DHCP_MAGIC_COOKIE_ARR => buf.skipn(4)?,
            Some(_) => return Err(BufferError::InvalidData),
            None => return Err(BufferError::BufTooShort),
        };

        // TODO (Techassi): Read DHCP options here
        let options = buf.read_vec(60)?;

        Ok(Self {
            header,
            addrs,
            sname,
            file,
            options,
        })
    }
}

impl Writeable for Message {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
        self.header.write::<E>(buf)?;
        self.addrs.write::<E>(buf)?;
        self.sname.write::<E>(buf)?;
        self.file.write::<E>(buf)?;

        // Write magic cookie
        buf.write_slice(constants::DHCP_MAGIC_COOKIE_ARR.as_slice())?;

        self.options.write::<E>(buf)
    }
}

impl Message {
    /// Create a new DHCP [`Message`]. Internally this creates a default header
    /// with a random transaction ID and then calls [`Self::new_with_header`].
    pub fn new() -> Self {
        let header = Header::new();
        return Self::new_with_header(header);
    }

    /// Create a new DHCP [`Message`] with the provided header. All other
    /// fields will use the default values.
    pub fn new_with_header(header: Header) -> Self {
        Self {
            header,
            ..Default::default()
        }
    }

    pub fn set_opcode() {}
}