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
    SubnetMask(Ipv4Addr),
    TimeOffset,
    Router(Vec<Ipv4Addr>),
    TimeServer,
    NameServer,
    DomainNameServer(Vec<Ipv4Addr>),
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
    IpAddrLeaseTime(u32),
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
    ServerIdentifier(Ipv4Addr),

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

impl Writeable for OptionData {
    type Error = OptionDataError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        let n = match self {
            OptionData::Pad => 0u8.write::<E>(buf)?,
            OptionData::End => 255u8.write::<E>(buf)?,
            OptionData::SubnetMask(mask) => mask.write::<E>(buf)?,
            OptionData::TimeOffset => todo!(),
            OptionData::Router(ips) => ips.write::<E>(buf)?,
            OptionData::TimeServer => todo!(),
            OptionData::NameServer => todo!(),
            OptionData::DomainNameServer(ips) => ips.write::<E>(buf)?,
            OptionData::LogServer => todo!(),
            OptionData::CookieServer => todo!(),
            OptionData::LprServer => todo!(),
            OptionData::ImpressServer => todo!(),
            OptionData::ResourceLocationServer => todo!(),
            OptionData::HostName(name) => name.write::<E>(buf)?,
            OptionData::BootFileSize => todo!(),
            OptionData::MeritDumpFile => todo!(),
            OptionData::DomainName => todo!(),
            OptionData::SwapServer => todo!(),
            OptionData::RootPath => todo!(),
            OptionData::ExtensionsPath => todo!(),
            OptionData::IpForwarding => todo!(),
            OptionData::NonLocalSourceRouting => todo!(),
            OptionData::PolicyFilter => todo!(),
            OptionData::MaxDatagramReassemblySize => todo!(),
            OptionData::DefaultIpTtl => todo!(),
            OptionData::PathMtuAgingTimeout => todo!(),
            OptionData::PathMtuPlateauTable => todo!(),
            OptionData::InterfaceMtu => todo!(),
            OptionData::AllSubnetsLocal => todo!(),
            OptionData::BroadcastAddr => todo!(),
            OptionData::PerformMaskDiscovery => todo!(),
            OptionData::MaskSupplier => todo!(),
            OptionData::PerformRouterDiscovery => todo!(),
            OptionData::RouterSolicitationAddr => todo!(),
            OptionData::StaticRoute => todo!(),
            OptionData::TrailerEncapsulation => todo!(),
            OptionData::ArpCacheTimeout => todo!(),
            OptionData::EthernetEncapsulation => todo!(),
            OptionData::TcpDefaultTtl => todo!(),
            OptionData::TcpKeepaliveInterval => todo!(),
            OptionData::TcpKeepaliveGarbage => todo!(),
            OptionData::NetworkInformationServiceDomain => todo!(),
            OptionData::NetworkInformationServers => todo!(),
            OptionData::NetworkTimeProtocolServers => todo!(),
            OptionData::VendorSpecificInformation => todo!(),
            OptionData::NetbiosNameServer => todo!(),
            OptionData::NetbiosDatagramDistributionServer => todo!(),
            OptionData::NetbiosNodeType => todo!(),
            OptionData::NetbiosScope => todo!(),
            OptionData::XWindowSystemFontServer => todo!(),
            OptionData::XWindowSystemDisplayManager => todo!(),
            OptionData::RequestedIpAddr(_) => todo!(),
            OptionData::IpAddrLeaseTime(time) => time.write::<E>(buf)?,
            OptionData::OptionOverload => todo!(),
            OptionData::DhcpMessageType(ty) => ty.write::<E>(buf)?,
            OptionData::ServerIdentifier(ip) => ip.write::<E>(buf)?,
            OptionData::ParameterRequestList(list) => list.write::<E>(buf)?,
            OptionData::Message => todo!(),
            OptionData::MaxDhcpMessageSize(size) => size.write::<E>(buf)?,
            OptionData::RenewalT1Time => todo!(),
            OptionData::RebindingT2Time => todo!(),
            OptionData::ClassIdentifier(_) => todo!(),
            OptionData::ClientIdentifier(_) => todo!(),
        };

        Ok(n)
    }
}

impl OptionData {
    pub fn read<E: Endianness>(
        buf: &mut ReadBuffer,
        header: &OptionHeader,
    ) -> Result<Self, OptionDataError> {
        let option_data = match header.tag {
            OptionTag::Pad => Self::Pad,
            OptionTag::End => Self::End,
            OptionTag::SubnetMask => Self::SubnetMask(Ipv4Addr::read::<E>(buf)?),
            OptionTag::TimeOffset => todo!(),
            OptionTag::Router => {
                if header.len % 4 != 0 {
                    return Err(OptionDataError::InvalidData);
                }

                let mut ips = Vec::new();

                for _ in 0..header.len / 4 {
                    ips.push(Ipv4Addr::read::<E>(buf)?);
                }

                Self::Router(ips)
            }
            OptionTag::TimeServer => todo!(),
            OptionTag::NameServer => todo!(),
            OptionTag::DomainNameServer => {
                if header.len % 4 != 0 {
                    return Err(OptionDataError::InvalidData);
                }

                let mut ips = Vec::new();

                for _ in 0..header.len / 4 {
                    ips.push(Ipv4Addr::read::<E>(buf)?);
                }

                Self::DomainNameServer(ips)
            }
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
            OptionTag::IpAddrLeaseTime => Self::IpAddrLeaseTime(u32::read::<E>(buf)?),
            OptionTag::OptionOverload => todo!(),
            OptionTag::DhcpMessageType => Self::DhcpMessageType(DhcpMessageType::read::<E>(buf)?),
            OptionTag::ServerIdentifier => Self::ServerIdentifier(Ipv4Addr::read::<E>(buf)?),
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

    pub fn len(&self) -> u8 {
        match self {
            OptionData::Pad => 1,
            OptionData::End => 1,
            OptionData::SubnetMask(_) => 4,
            OptionData::TimeOffset => 4,
            OptionData::Router(ips) => ips.len() as u8,
            OptionData::TimeServer => todo!(),
            OptionData::NameServer => todo!(),
            OptionData::DomainNameServer(ips) => ips.len() as u8,
            OptionData::LogServer => todo!(),
            OptionData::CookieServer => todo!(),
            OptionData::LprServer => todo!(),
            OptionData::ImpressServer => todo!(),
            OptionData::ResourceLocationServer => todo!(),
            OptionData::HostName(h) => h.len() as u8,
            OptionData::BootFileSize => 2,
            OptionData::MeritDumpFile => todo!(),
            OptionData::DomainName => todo!(),
            OptionData::SwapServer => todo!(),
            OptionData::RootPath => todo!(),
            OptionData::ExtensionsPath => todo!(),
            OptionData::IpForwarding => 1,
            OptionData::NonLocalSourceRouting => 1,
            OptionData::PolicyFilter => todo!(),
            OptionData::MaxDatagramReassemblySize => 2,
            OptionData::DefaultIpTtl => 1,
            OptionData::PathMtuAgingTimeout => 4,
            OptionData::PathMtuPlateauTable => todo!(),
            OptionData::InterfaceMtu => 2,
            OptionData::AllSubnetsLocal => 1,
            OptionData::BroadcastAddr => 4,
            OptionData::PerformMaskDiscovery => 1,
            OptionData::MaskSupplier => 1,
            OptionData::PerformRouterDiscovery => 1,
            OptionData::RouterSolicitationAddr => 4,
            OptionData::StaticRoute => todo!(),
            OptionData::TrailerEncapsulation => 1,
            OptionData::ArpCacheTimeout => 4,
            OptionData::EthernetEncapsulation => 1,
            OptionData::TcpDefaultTtl => 1,
            OptionData::TcpKeepaliveInterval => 4,
            OptionData::TcpKeepaliveGarbage => 1,
            OptionData::NetworkInformationServiceDomain => todo!(),
            OptionData::NetworkInformationServers => todo!(),
            OptionData::NetworkTimeProtocolServers => todo!(),
            OptionData::VendorSpecificInformation => todo!(),
            OptionData::NetbiosNameServer => todo!(),
            OptionData::NetbiosDatagramDistributionServer => todo!(),
            OptionData::NetbiosNodeType => 1,
            OptionData::NetbiosScope => todo!(),
            OptionData::XWindowSystemFontServer => todo!(),
            OptionData::XWindowSystemDisplayManager => todo!(),
            OptionData::RequestedIpAddr(_) => 4,
            OptionData::IpAddrLeaseTime(_) => 4,
            OptionData::OptionOverload => 1,
            OptionData::DhcpMessageType(_) => 1,
            OptionData::ServerIdentifier(_) => 4,
            OptionData::ParameterRequestList(l) => l.len() as u8,
            OptionData::Message => todo!(),
            OptionData::MaxDhcpMessageSize(_) => 2,
            OptionData::RenewalT1Time => 4,
            OptionData::RebindingT2Time => 4,
            OptionData::ClassIdentifier(_) => todo!(),
            OptionData::ClientIdentifier(_) => todo!(),
        }
    }
}
