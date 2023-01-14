use binbuf::prelude::*;
use thiserror::Error;

use crate::types::{options::DhcpMessageType, OptionHeader, OptionTag};

#[derive(Debug, Error)]
pub enum OptionDataError {
    #[error("Invalid option data")]
    InvalidData,

    #[error("IO error: {0}")]
    Io(#[from] BufferError),
}

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
    HostName,
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
    RequestedIpAddr,
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
    ParameterRequestList,
    Message,
    MaxDhcpMessageSize,
    RenewalT1Time,
    RebindingT2Time,
    ClassIdentifier,

    /// #### Client-identifier
    ///
    /// ```text
    /// Code   Len   Type  Client-Identifier
    /// +-----+-----+-----+-----+-----+---
    /// |  61 |  n  |  t1 |  i1 |  i2 | ...
    /// +-----+-----+-----+-----+-----+---
    /// ```
    ClientIdentifier,
}

impl OptionData {
    pub fn read<E: Endianness>(
        buf: &mut impl ToReadBuffer,
        header: &OptionHeader,
    ) -> Result<Self, OptionDataError> {
        match header.tag {
            OptionTag::Pad => todo!(),
            OptionTag::End => todo!(),
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
            OptionTag::HostName => todo!(),
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
            OptionTag::RequestedIpAddr => todo!(),
            OptionTag::IpAddrLeaseTime => todo!(),
            OptionTag::OptionOverload => todo!(),
            OptionTag::DhcpMessageType => todo!(),
            OptionTag::ServerIdentifier => todo!(),
            OptionTag::ParameterRequestList => todo!(),
            OptionTag::Message => todo!(),
            OptionTag::MaxDhcpMessageSize => todo!(),
            OptionTag::RenewalT1Time => todo!(),
            OptionTag::RebindingT2Time => todo!(),
            OptionTag::ClassIdentifier => todo!(),
            OptionTag::ClientIdentifier => todo!(),
        }
    }
}
