use std::{fmt::Display, net::Ipv4Addr};

use binbuf::prelude::*;
use thiserror::Error;

use crate::{
    constants,
    types::{
        options::DhcpMessageType, DhcpOption, HardwareAddr, Header, HeaderError, OptionData,
        OptionError, OptionTag,
    },
};

#[derive(Debug, Error)]
pub enum MessageError {
    #[error("Header error: {0}")]
    HeaderError(#[from] HeaderError),

    #[error("Option error: {0}")]
    OptionError(#[from] OptionError),

    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError),

    #[error("Option with tag {0} already ppresent, duplicates are not allowed")]
    DuplicateOptionError(OptionTag),
}

/// [`Message`] describes a complete DHCP message. The same packet field
/// layout is used in both directions.
///
/// ### See
///
/// RFC 2131 - Section 2 - Protocol Summary: https://datatracker.ietf.org/doc/html/rfc2131#section-2
#[derive(Debug)]
pub struct Message {
    /// Header fields like the opcode, transaction id and additional flags.
    pub header: Header,

    /// Client IP address; filled in by client in BOOTREQUEST if known.
    pub ciaddr: Ipv4Addr,

    /// 'Your' (client) IP address. Filled by server if client doesn't know
    /// its own address (ciaddr was 0).
    pub yiaddr: Ipv4Addr,

    /// Server IP address. Returned in BOOTREPLY by server.
    pub siaddr: Ipv4Addr,

    /// Gateway IP address, used in optional cross-gateway booting.
    pub giaddr: Ipv4Addr,

    /// Client hardware address, filled in by client (16 octets).
    pub chaddr: HardwareAddr,

    /// Optional server host name, null terminated string (64 octets).
    pub sname: Vec<u8>,

    /// Boot file name, null terminated string. 'Generic' name or null in
    /// BOOTREQUEST, fully qualified directory-path name in bootreply
    /// (128 octets).
    pub file: Vec<u8>,

    /// Originally this was called 'vendor extensions' in the BOOTP RFC. The
    /// RFC states:
    ///
    /// Optional vendor-specific area, e.g. could be hardware type/serial on
    /// request, or 'capability' / remote file system handle on reply. This
    /// info may be set aside for use by a third phase bootstrap or kernel
    /// (64 octets).
    ///
    /// The DHCP RFC renames this filed to 'options'.
    pub options: Vec<DhcpOption>,
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let options: String = self
            .options
            .iter()
            .map(|o| format!(";; {:?}\n", o))
            .collect();

        write!(
            f,
            ";; ->>HEADER<<- MT: {}, HT: {}, HWADDR LEN: {}, HOPS: {}, XID: {:#X}\n\
            ;; SECS: {}, FLAGS: {}\n\n\
            ;; ->>ADDRS<<-\n\
            ;; Client IP address: {}\n\
            ;; Your (client) IP address: {}\n\
            ;; Next server IP address: {}\n\
            ;; Relay agent IP address: {}\n\
            ;; Client MAC address: {}\n\n\
            ;; ->>OPTIONS<<-\n{}",
            self.header.opcode,
            self.header.htype,
            self.header.hlen,
            self.header.hops,
            self.header.xid,
            self.header.secs,
            self.header.flags,
            self.ciaddr,
            self.yiaddr,
            self.siaddr,
            self.giaddr,
            self.chaddr,
            options
        )
    }
}

impl Default for Message {
    fn default() -> Self {
        Self {
            header: Header::default(),
            ciaddr: Ipv4Addr::new(0, 0, 0, 0),
            yiaddr: Ipv4Addr::new(0, 0, 0, 0),
            siaddr: Ipv4Addr::new(0, 0, 0, 0),
            giaddr: Ipv4Addr::new(0, 0, 0, 0),
            chaddr: Default::default(),
            sname: vec![0; 64],
            file: vec![0; 128],
            options: vec![],
        }
    }
}

impl Readable for Message {
    type Error = MessageError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        let header = Header::read::<E>(buf)?;

        let ciaddr = Ipv4Addr::read::<E>(buf)?;
        let yiaddr = Ipv4Addr::read::<E>(buf)?;
        let siaddr = Ipv4Addr::read::<E>(buf)?;
        let giaddr = Ipv4Addr::read::<E>(buf)?;
        let chaddr = HardwareAddr::read::<E>(buf, header.hlen)?;

        let sname = buf.read_vec(64)?;
        let file = buf.read_vec(128)?;

        match buf.peekn::<4>() {
            Some(m) if m == constants::DHCP_MAGIC_COOKIE_ARR => buf.skipn(4)?,
            Some(_) => return Err(MessageError::BufferError(BufferError::InvalidData)),
            None => return Err(MessageError::BufferError(BufferError::BufTooShort)),
        };

        let options = read_options::<E>(buf)?;

        Ok(Self {
            header,
            ciaddr,
            yiaddr,
            siaddr,
            giaddr,
            chaddr,
            sname,
            file,
            options,
        })
    }
}

fn read_options<E: Endianness>(buf: &mut ReadBuffer) -> Result<Vec<DhcpOption>, MessageError> {
    if buf.is_empty() {
        return Err(MessageError::BufferError(BufferError::BufTooShort));
    }

    let mut options = vec![];

    while !buf.is_empty() {
        let option = match DhcpOption::read::<E>(buf) {
            Ok(option) => option,
            Err(err) => return Err(MessageError::OptionError(err)),
        };
        options.push(option);
    }

    Ok(options)
}

impl Writeable for Message {
    type Error = MessageError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        // let n = bytes_written! {};

        let mut n = 0;

        n += self.header.write::<E>(buf)?;
        n += self.ciaddr.write::<E>(buf)?;
        n += self.yiaddr.write::<E>(buf)?;
        n += self.siaddr.write::<E>(buf)?;
        n += self.giaddr.write::<E>(buf)?;
        n += self.chaddr.write::<E>(buf)?;
        n += self.sname.write::<E>(buf)?;
        n += self.file.write::<E>(buf)?;

        // Write magic cookie
        n += buf.write(constants::DHCP_MAGIC_COOKIE_ARR);

        n += self.options.write::<E>(buf)?;

        Ok(n)
    }
}

impl Message {
    /// Create a new DHCP [`Message`]. Internally this creates a default header
    /// with a random transaction ID and then calls [`Self::new_with_header`].
    pub fn new() -> Self {
        let header = Header::new();
        Self::new_with_header(header)
    }

    /// Create a new DHCP [`Message`] with the provided transaction id (xid).
    pub fn new_with_xid(xid: u32) -> Self {
        let header = Header::new_with_xid(xid);
        Self::new_with_header(header)
    }

    /// Create a new DHCP [`Message`] with the provided header. All other
    /// fields will use the default values.
    pub fn new_with_header(header: Header) -> Self {
        Self {
            header,
            ..Default::default()
        }
    }

    /// Get DHCP message type
    pub fn get_message_type(&self) -> Option<&DhcpMessageType> {
        for option in &self.options {
            if option.header().tag == OptionTag::DhcpMessageType {
                match option.data() {
                    OptionData::DhcpMessageType(ty) => return Some(ty),
                    _ => return None,
                }
            }
        }

        None
    }

    pub fn set_hardware_address(&mut self, haddr: HardwareAddr) {
        // TODO (Techassi): We should return a u8. This would make the len call falliable tho
        self.header.hlen = haddr.len() as u8;
        self.chaddr = haddr;
    }

    pub fn add_option(&mut self, option: DhcpOption) -> Result<(), MessageError> {
        // TODO (Techassi): We should probably make the options field a HashMap
        for opt in &self.options {
            if opt.header().tag == option.header().tag {
                return Err(MessageError::DuplicateOptionError(
                    option.header().tag.clone(),
                ));
            }
        }

        self.options.push(option);
        Ok(())
    }
}
