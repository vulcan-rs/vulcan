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
    MINIMUM_LEGAL_MAX_MESSAGE_SIZE,
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
    TimeOffset(u32),
    Router(Vec<Ipv4Addr>),
    TimeServer(Vec<Ipv4Addr>),
    NameServer(Vec<Ipv4Addr>),
    DomainNameServer(Vec<Ipv4Addr>),
    LogServer(Vec<Ipv4Addr>),
    CookieServer(Vec<Ipv4Addr>),
    LprServer(Vec<Ipv4Addr>),
    ImpressServer(Vec<Ipv4Addr>),
    ResourceLocationServer(Vec<Ipv4Addr>),
    HostName(String),
    BootFileSize(u16),
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
    RenewalT1Time(u32),
    RebindingT2Time(u32),

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
            OptionData::TimeOffset(off) => off.write::<E>(buf)?,
            OptionData::Router(ips) => ips.write::<E>(buf)?,
            OptionData::TimeServer(ips) => ips.write::<E>(buf)?,
            OptionData::NameServer(ips) => ips.write::<E>(buf)?,
            OptionData::DomainNameServer(ips) => ips.write::<E>(buf)?,
            OptionData::LogServer(ips) => ips.write::<E>(buf)?,
            OptionData::CookieServer(ips) => ips.write::<E>(buf)?,
            OptionData::LprServer(ips) => ips.write::<E>(buf)?,
            OptionData::ImpressServer(ips) => ips.write::<E>(buf)?,
            OptionData::ResourceLocationServer(ips) => ips.write::<E>(buf)?,
            OptionData::HostName(name) => name.write::<E>(buf)?,
            OptionData::BootFileSize(size) => size.write::<E>(buf)?,
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
            OptionData::RequestedIpAddr(ip) => ip.write::<E>(buf)?,
            OptionData::IpAddrLeaseTime(time) => time.write::<E>(buf)?,
            OptionData::OptionOverload => todo!(),
            OptionData::DhcpMessageType(ty) => ty.write::<E>(buf)?,
            OptionData::ServerIdentifier(ip) => ip.write::<E>(buf)?,
            OptionData::ParameterRequestList(list) => list.write::<E>(buf)?,
            OptionData::Message => todo!(),
            OptionData::MaxDhcpMessageSize(size) => size.write::<E>(buf)?,
            OptionData::RenewalT1Time(time) => time.write::<E>(buf)?,
            OptionData::RebindingT2Time(time) => time.write::<E>(buf)?,
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
            OptionTag::TimeOffset => Self::TimeOffset(u32::read::<E>(buf)?),
            OptionTag::Router => {
                let ips = read_ip_addrs_set::<E>(buf, header.len)?;
                Self::Router(ips)
            }
            OptionTag::TimeServer => {
                let ips = read_ip_addrs_set::<E>(buf, header.len)?;
                Self::TimeServer(ips)
            }
            OptionTag::NameServer => {
                let ips = read_ip_addrs_set::<E>(buf, header.len)?;
                Self::NameServer(ips)
            }
            OptionTag::DomainNameServer => {
                let ips = read_ip_addrs_set::<E>(buf, header.len)?;
                Self::DomainNameServer(ips)
            }
            OptionTag::LogServer => {
                let ips = read_ip_addrs_set::<E>(buf, header.len)?;
                Self::LogServer(ips)
            }
            OptionTag::CookieServer => {
                let ips = read_ip_addrs_set::<E>(buf, header.len)?;
                Self::CookieServer(ips)
            }
            OptionTag::LprServer => {
                let ips = read_ip_addrs_set::<E>(buf, header.len)?;
                Self::LprServer(ips)
            }
            OptionTag::ImpressServer => {
                let ips = read_ip_addrs_set::<E>(buf, header.len)?;
                Self::ImpressServer(ips)
            }
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

                if size < MINIMUM_LEGAL_MAX_MESSAGE_SIZE {
                    return Err(OptionDataError::InvalidDhcpMessageSize);
                }

                Self::MaxDhcpMessageSize(size)
            }
            OptionTag::RenewalT1Time => Self::RenewalT1Time(u32::read::<E>(buf)?),
            OptionTag::RebindingT2Time => Self::RebindingT2Time(u32::read::<E>(buf)?),
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

    pub fn size(&self) -> u8 {
        match self {
            OptionData::Pad => 1,
            OptionData::End => 1,
            OptionData::SubnetMask(_) => 4,
            OptionData::TimeOffset(_) => 4,
            OptionData::Router(ips) => (ips.len() * 4) as u8,
            OptionData::TimeServer(ips) => (ips.len() * 4) as u8,
            OptionData::NameServer(ips) => (ips.len() * 4) as u8,
            OptionData::DomainNameServer(ips) => (ips.len() * 4) as u8,
            OptionData::LogServer(ips) => (ips.len() * 4) as u8,
            OptionData::CookieServer(ips) => (ips.len() * 4) as u8,
            OptionData::LprServer(ips) => (ips.len() * 4) as u8,
            OptionData::ImpressServer(ips) => (ips.len() * 4) as u8,
            OptionData::ResourceLocationServer(ips) => (ips.len() * 4) as u8,
            OptionData::HostName(h) => h.len() as u8,
            OptionData::BootFileSize(_) => 2,
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
            OptionData::RenewalT1Time(_) => 4,
            OptionData::RebindingT2Time(_) => 4,
            OptionData::ClassIdentifier(_) => todo!(),
            OptionData::ClientIdentifier(_) => todo!(),
        }
    }
}

/// Reads a set of IPv4 addresses. This function ensures that the provided
/// length is at least 4 and a multiple of 4.
fn read_ip_addrs_set<E: Endianness>(
    buf: &mut ReadBuffer,
    len: u8,
) -> Result<Vec<Ipv4Addr>, OptionDataError> {
    if len < 4 || len % 4 != 0 {
        return Err(OptionDataError::InvalidData);
    }

    let mut ips = Vec::new();

    for _ in 0..len / 4 {
        ips.push(Ipv4Addr::read::<E>(buf)?);
    }

    Ok(ips)
}
