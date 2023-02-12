use std::net::Ipv4Addr;

use binbuf::prelude::*;
use thiserror::Error;

use crate::{
    types::{
        options::{
            ClassIdentifier, ClientIdentifier, DhcpMessageType, ParameterRequestList,
            ParameterRequestListError,
        },
        OptionHeader, OptionTag,
    },
    DHCP_MINIMUM_LEGAL_MAX_MESSAGE_SIZE,
};

#[derive(Debug, Error)]
pub enum OptionDataError {
    #[error("Invalid DHCP message size")]
    InvalidDhcpMessageSize,

    #[error("Invalid option data")]
    InvalidData,

    #[error("Parameter request list error: {0}")]
    ParameterRequestListError(#[from] ParameterRequestListError),

    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError),
}

#[derive(Debug)]
pub enum OptionData {
    Pad,
    End,
    SubnetMask,
    TimeOffset,
    Router,
    TimeServer,
    NameServer,
    DomainNameServer,
    LogServer,
    CookieServer,
    LprServer,
    ImpressServer,
    ResourceLocationServer,
    HostName(String),
    BootFileSize,
    MeritDumpFile,
    DomainName,
    SwapServer,
    RootPath,
    ExtensionsPath,
    IpForwarding,
    NonLocalSourceRouting,
    PolicyFilter,
    MaxDatagramReassemblySize,
    DefaultIpTtl,
    PathMtuAgingTimeout,
    PathMtuPlateauTable,
    InterfaceMtu,
    AllSubnetsLocal,
    BroadcastAddr,
    PerformMaskDiscovery,
    MaskSupplier,
    PerformRouterDiscovery,
    RouterSolicitationAddr,
    StaticRoute,
    TrailerEncapsulation,
    ArpCacheTimeout,
    EthernetEncapsulation,
    TcpDefaultTtl,
    TcpKeepaliveInterval,
    TcpKeepaliveGarbage,
    NetworkInformationServiceDomain,
    NetworkInformationServers,
    NetworkTimeProtocolServers,
    VendorSpecificInformation,
    NetbiosNameServer,
    NetbiosDatagramDistributionServer,
    NetbiosNodeType,
    NetbiosScope,
    XWindowSystemFontServer,
    XWindowSystemDisplayManager,

    /// #### Requested IP Address
    ///
    /// The code for this option is 50, and its length is 4.
    ///
    /// ```text
    /// Code   Len          Address
    /// +-----+-----+-----+-----+-----+-----+
    /// |  50 |  4  |  a1 |  a2 |  a3 |  a4 |
    /// +-----+-----+-----+-----+-----+-----+
    /// ```
    RequestedIpAddr(Ipv4Addr),
    IpAddrLeaseTime,
    OptionOverload,
    /// #### DHCP Message Type
    ///
    /// ```text
    ///  Code   Len  Type
    /// +-----+-----+-----+
    /// |  53 |  1  | 1-7 |
    /// +-----+-----+-----+
    /// ```
    DhcpMessageType(DhcpMessageType),
    ServerIdentifier,

    /// #### Parameter Request List
    ///
    /// The code for this option is 55. Its minimum length is 1.
    ///
    /// ```text
    ///  Code   Len   Option Codes
    /// +-----+-----+-----+-----+---
    /// |  55 |  n  |  c1 |  c2 | ...
    /// +-----+-----+-----+-----+---
    /// ```
    ParameterRequestList(ParameterRequestList),
    Message,

    /// #### Maximum DHCP Message Size
    ///
    /// The code for this option is 57, and its length is 2. The minimum legal
    /// value is 576 octets.
    ///
    /// ```text
    ///  Code   Len     Length
    /// +-----+-----+-----+-----+
    /// |  57 |  2  |  l1 |  l2 |
    /// +-----+-----+-----+-----+
    /// ```
    MaxDhcpMessageSize(u16),
    RenewalT1Time,
    RebindingT2Time,

    /// #### Class-identifier
    ///
    /// The code for this option is 60, and its minimum length is 1.
    ///
    /// ```text
    /// Code   Len   Class-Identifier
    /// +-----+-----+-----+-----+---
    /// |  60 |  n  |  i1 |  i2 | ...
    /// +-----+-----+-----+-----+---
    /// ```
    ClassIdentifier(ClassIdentifier),

    /// #### Client-identifier
    ///
    /// ```text
    /// Code   Len   Type  Client-Identifier
    /// +-----+-----+-----+-----+-----+---
    /// |  61 |  n  |  t1 |  i1 |  i2 | ...
    /// +-----+-----+-----+-----+-----+---
    /// ```
    ClientIdentifier(ClientIdentifier),
}

impl OptionData {
    pub fn read<E: Endianness>(
        buf: &mut ReadBuffer,
        header: &OptionHeader,
    ) -> Result<Self, OptionDataError> {
        let option_data = match header.tag {
            OptionTag::Pad => Self::Pad,
            OptionTag::End => Self::End,
            OptionTag::SubnetMask => todo!(),
            OptionTag::TimeOffset => todo!(),
            OptionTag::Router => todo!(),
            OptionTag::TimeServer => todo!(),
            OptionTag::NameServer => todo!(),
            OptionTag::DomainNameServer => todo!(),
            OptionTag::LogServer => todo!(),
            OptionTag::CookieServer => todo!(),
            OptionTag::LprServer => todo!(),
            OptionTag::ImpressServer => todo!(),
            OptionTag::ResourceLocationServer => todo!(),
            OptionTag::HostName => {
                let b = buf.read_vec(header.len as usize)?;
                Self::HostName(String::from_utf8(b).unwrap())
            }
            OptionTag::BootFileSize => todo!(),
            OptionTag::MeritDumpFile => todo!(),
            OptionTag::DomainName => todo!(),
            OptionTag::SwapServer => todo!(),
            OptionTag::RootPath => todo!(),
            OptionTag::ExtensionsPath => todo!(),
            OptionTag::IpForwarding => todo!(),
            OptionTag::NonLocalSourceRouting => todo!(),
            OptionTag::PolicyFilter => todo!(),
            OptionTag::MaxDatagramReassemblySize => todo!(),
            OptionTag::DefaultIpTtl => todo!(),
            OptionTag::PathMtuAgingTimeout => todo!(),
            OptionTag::PathMtuPlateauTable => todo!(),
            OptionTag::InterfaceMtu => todo!(),
            OptionTag::AllSubnetsLocal => todo!(),
            OptionTag::BroadcastAddr => todo!(),
            OptionTag::PerformMaskDiscovery => todo!(),
            OptionTag::MaskSupplier => todo!(),
            OptionTag::PerformRouterDiscovery => todo!(),
            OptionTag::RouterSolicitationAddr => todo!(),
            OptionTag::StaticRoute => todo!(),
            OptionTag::TrailerEncapsulation => todo!(),
            OptionTag::ArpCacheTimeout => todo!(),
            OptionTag::EthernetEncapsulation => todo!(),
            OptionTag::TcpDefaultTtl => todo!(),
            OptionTag::TcpKeepaliveInterval => todo!(),
            OptionTag::TcpKeepaliveGarbage => todo!(),
            OptionTag::NetworkInformationServiceDomain => todo!(),
            OptionTag::NetworkInformationServers => todo!(),
            OptionTag::NetworkTimeProtocolServers => todo!(),
            OptionTag::VendorSpecificInformation => todo!(),
            OptionTag::NetbiosNameServer => todo!(),
            OptionTag::NetbiosDatagramDistributionServer => todo!(),
            OptionTag::NetbiosNodeType => todo!(),
            OptionTag::NetbiosScope => todo!(),
            OptionTag::XWindowSystemFontServer => todo!(),
            OptionTag::XWindowSystemDisplayManager => todo!(),
            OptionTag::RequestedIpAddr => Self::RequestedIpAddr(Ipv4Addr::read::<E>(buf)?),
            OptionTag::IpAddrLeaseTime => todo!(),
            OptionTag::OptionOverload => todo!(),
            OptionTag::DhcpMessageType => Self::DhcpMessageType(DhcpMessageType::read::<E>(buf)?),
            OptionTag::ServerIdentifier => todo!(),
            OptionTag::ParameterRequestList => {
                Self::ParameterRequestList(ParameterRequestList::read::<E>(buf, header.len)?)
            }
            OptionTag::Message => todo!(),
            OptionTag::MaxDhcpMessageSize => {
                let size = u16::read::<E>(buf)?;

                if size < DHCP_MINIMUM_LEGAL_MAX_MESSAGE_SIZE {
                    return Err(OptionDataError::InvalidDhcpMessageSize);
                }

                Self::MaxDhcpMessageSize(size)
            }
            OptionTag::RenewalT1Time => todo!(),
            OptionTag::RebindingT2Time => todo!(),
            OptionTag::ClassIdentifier => {
                Self::ClassIdentifier(ClassIdentifier::read::<E>(buf, header.len)?)
            }
            OptionTag::ClientIdentifier => {
                Self::ClientIdentifier(ClientIdentifier::read::<E>(buf, header.len)?)
            }
            OptionTag::DhcpCaptivePortal => todo!(),
            OptionTag::UnassignedOrRemoved(_) => todo!(),
        };

        Ok(option_data)
    }
}

impl Writeable for OptionData {
    type Error = BufferError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        todo!()
    }
}
