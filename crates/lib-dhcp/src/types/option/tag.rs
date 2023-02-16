use std::fmt::Display;

use binbuf::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OptionTagError {
    #[error("Invalid option tag: {0}")]
    InvalidTag(u8),

    #[error("Buffer error: {0}")]
    BufferError(#[from] BufferError),
}

#[derive(Debug, PartialEq, Clone)]
pub enum OptionTag {
    /// See [3.1. Pad Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.1)
    Pad,

    /// See [3.2. End Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.2)
    End,

    /// See
    /// - [3.3. Subnet Mask](https://datatracker.ietf.org/doc/html/rfc1533#section-3.3)
    /// - [RFC 950](https://datatracker.ietf.org/doc/html/rfc950)
    SubnetMask,

    /// See [3.4. Time Offset](https://datatracker.ietf.org/doc/html/rfc1533#section-3.4)
    TimeOffset,

    /// See [3.5. Router Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.5)
    Router,

    /// See
    /// - [3.6. Time Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.6)
    /// - [RFC 868](https://datatracker.ietf.org/doc/html/rfc868)
    TimeServer,

    /// See [3.7. Name Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.7)
    NameServer,

    /// See
    /// - [3.8. Domain Name Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.8)
    /// - [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035)
    DomainNameServer,

    /// See [3.9. Log Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.9)
    LogServer,

    /// See
    /// - [3.10. Cookie Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.10)
    /// - [RFC 865](https://datatracker.ietf.org/doc/html/rfc865)
    CookieServer,

    /// See
    /// - [3.11. LPR Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.11)
    /// - [RFC 1179](https://datatracker.ietf.org/doc/html/rfc1179)
    LprServer,

    /// See [3.12. Impress Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.12)
    ImpressServer,

    /// See
    /// - [3.13. Resource Location Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.13)
    /// - [RFC 887](https://datatracker.ietf.org/doc/html/rfc887)
    ResourceLocationServer,

    /// See
    /// - [3.14. Host Name Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.14)
    /// - [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035)
    HostName,

    /// See [3.15. Boot File Size Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.15)
    BootFileSize,

    /// See [3.16. Merit Dump File](https://datatracker.ietf.org/doc/html/rfc1533#section-3.16)
    MeritDumpFile,

    /// See [3.17. Domain Name](https://datatracker.ietf.org/doc/html/rfc1533#section-3.17)
    DomainName,

    /// See [3.18. Swap Server](https://datatracker.ietf.org/doc/html/rfc1533#section-3.18)
    SwapServer,

    /// See [3.19. Root Path](https://datatracker.ietf.org/doc/html/rfc1533#section-3.19)
    RootPath,

    /// - the length of the file is unconstrained;
    /// - all references to Tag 18 (i.e., instances of the BOOTP Extensions

    /// See [3.20. Extensions Path](https://datatracker.ietf.org/doc/html/rfc1533#section-3.20)
    ExtensionsPath,

    /// See [4.1. IP Forwarding Enable/Disable Option](https://datatracker.ietf.org/doc/html/rfc1533#section-4.1)
    IpForwarding,

    /// See [4.2. Non-Local Source Routing Enable/Disable Option][2]
    NonLocalSourceRouting,

    /// See [[4][1]] for further information.

    /// See [4.3. Policy Filter Option][2]
    PolicyFilter,

    /// See [4.4. Maximum Datagram Reassembly Size][1]
    MaxDatagramReassemblySize,

    /// See [4.5. Default IP Time-to-live][1]
    DefaultIpTtl,

    /// See [4.6. Path MTU Aging Timeout Option][3]
    PathMtuAgingTimeout,

    /// See [4.7. Path MTU Plateau Table Option][²]
    PathMtuPlateauTable,

    /// See [5.1. Interface MTU Option][1]
    InterfaceMtu,

    /// See [5.2. All Subnets are Local Option][1]
    AllSubnetsLocal,

    /// See [5.3. Broadcast Address Option][2]
    BroadcastAddr,

    /// See [5.4. Perform Mask Discovery Option][1]
    PerformMaskDiscovery,

    /// See [5.5. Mask Supplier Option][1]
    MaskSupplier,

    /// See [5.6. Perform Router Discovery Option][3]
    PerformRouterDiscovery,

    /// See [5.7. Router Solicitation Address Option][1]
    RouterSolicitationAddr,

    /// See [5.8. Static Route Option][2]
    StaticRoute,

    /// See [6.1. Trailer Encapsulation Option][3]
    TrailerEncapsulation,

    /// See [6.2. ARP Cache Timeout Option][1]
    ArpCacheTimeout,

    /// See [6.3. Ethernet Encapsulation Option][5]
    EthernetEncapsulation,

    /// See [7.1. TCP Default TTL Option][1]
    TcpDefaultTtl,

    /// See [7.2. TCP Keepalive Interval Option][1]
    TcpKeepaliveInterval,

    /// See [7.3. TCP Keepalive Garbage Option][1]
    TcpKeepaliveGarbage,

    /// See [8.1. Network Information Service Domain Option][2]
    NetworkInformationServiceDomain,

    /// See [8.2. Network Information Servers Option][1]
    NetworkInformationServers,

    /// See [8.3. Network Time Protocol Servers Option][2]
    NetworkTimeProtocolServers,

    /// See [8.4. Vendor Specific Information][2]
    VendorSpecificInformation,

    /// See [8.5. NetBIOS over TCP/IP Name Server Option][4]
    NetbiosNameServer,

    /// See [8.6. NetBIOS over TCP/IP Datagram Distribution Server Option][2]
    NetbiosDatagramDistributionServer,

    /// - `0x1` -  B-node
    /// - `0x2` -  P-node
    /// - `0x4` -  M-node
    /// - `0x8` -  H-node

    /// See [8.7. NetBIOS over TCP/IP Node Type Option][2]
    NetbiosNodeType,

    /// See [8.8. NetBIOS over TCP/IP Scope Option][5]
    NetbiosScope,

    /// See [8.9. X Window System Font Server Option][2]
    XWindowSystemFontServer,

    XWindowSystemDisplayManager,

    /// See [9.1. Requested IP Address][1]
    RequestedIpAddr,

    /// See [9.2. IP Address Lease Time][1]
    IpAddrLeaseTime,

    /// See [9.3. Option Overload][1]
    OptionOverload,

    /// See [9.4. DHCP Message Type][1]
    DhcpMessageType,

    /// See [9.5. Server Identifier][1]
    ServerIdentifier,

    /// See [9.6. Parameter Request List][1]
    ParameterRequestList,

    /// See [9.7. Message][1]
    Message,

    /// See [9.8. Maximum DHCP Message Size][1]
    MaxDhcpMessageSize,

    /// See [9.9. Renewal (T1) Time Value][1]
    RenewalT1Time,

    /// See [9.10. Rebinding (T2) Time Value][1]
    RebindingT2Time,

    /// See [9.11. Class-identifier][1]
    ClassIdentifier,

    /// See [9.12. Client-identifier][1]
    ClientIdentifier,

    /// See [Captive-Portal Identification in DHCP and Router Advertisements (RAs)][2]
    DhcpCaptivePortal,

    UnassignedOrRemoved(u8),
}

impl Display for OptionTag {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tag = u8::from(self);
        write!(f, "{}", tag)
    }
}

impl TryFrom<u8> for OptionTag {
    type Error = OptionTagError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Pad),
            1 => Ok(Self::SubnetMask),
            2 => Ok(Self::TimeOffset),
            3 => Ok(Self::Router),
            4 => Ok(Self::TimeServer),
            5 => Ok(Self::NameServer),
            6 => Ok(Self::DomainNameServer),
            7 => Ok(Self::LogServer),
            8 => Ok(Self::CookieServer),
            9 => Ok(Self::LprServer),
            10 => Ok(Self::ImpressServer),
            11 => Ok(Self::ResourceLocationServer),
            12 => Ok(Self::HostName),
            13 => Ok(Self::BootFileSize),
            14 => Ok(Self::MeritDumpFile),
            15 => Ok(Self::DomainName),
            16 => Ok(Self::SwapServer),
            17 => Ok(Self::RootPath),
            18 => Ok(Self::ExtensionsPath),
            19 => Ok(Self::IpForwarding),
            20 => Ok(Self::NonLocalSourceRouting),
            21 => Ok(Self::PolicyFilter),
            22 => Ok(Self::MaxDatagramReassemblySize),
            23 => Ok(Self::DefaultIpTtl),
            24 => Ok(Self::PathMtuAgingTimeout),
            25 => Ok(Self::PathMtuPlateauTable),
            26 => Ok(Self::InterfaceMtu),
            27 => Ok(Self::AllSubnetsLocal),
            28 => Ok(Self::BroadcastAddr),
            29 => Ok(Self::PerformMaskDiscovery),
            30 => Ok(Self::MaskSupplier),
            31 => Ok(Self::PerformRouterDiscovery),
            32 => Ok(Self::RouterSolicitationAddr),
            33 => Ok(Self::StaticRoute),
            34 => Ok(Self::TrailerEncapsulation),
            35 => Ok(Self::ArpCacheTimeout),
            36 => Ok(Self::EthernetEncapsulation),
            37 => Ok(Self::TcpDefaultTtl),
            38 => Ok(Self::TcpKeepaliveInterval),
            39 => Ok(Self::TcpKeepaliveGarbage),
            40 => Ok(Self::NetworkInformationServiceDomain),
            41 => Ok(Self::NetworkInformationServers),
            42 => Ok(Self::NetworkTimeProtocolServers),
            43 => Ok(Self::VendorSpecificInformation),
            44 => Ok(Self::NetbiosNameServer),
            45 => Ok(Self::NetbiosDatagramDistributionServer),
            46 => Ok(Self::NetbiosNodeType),
            47 => Ok(Self::NetbiosScope),
            48 => Ok(Self::XWindowSystemFontServer),
            49 => Ok(Self::XWindowSystemDisplayManager),
            50 => Ok(Self::RequestedIpAddr),
            51 => Ok(Self::IpAddrLeaseTime),
            52 => Ok(Self::OptionOverload),
            53 => Ok(Self::DhcpMessageType),
            54 => Ok(Self::ServerIdentifier),
            55 => Ok(Self::ParameterRequestList),
            56 => Ok(Self::Message),
            57 => Ok(Self::MaxDhcpMessageSize),
            58 => Ok(Self::RenewalT1Time),
            59 => Ok(Self::RebindingT2Time),
            60 => Ok(Self::ClassIdentifier),
            61 => Ok(Self::ClientIdentifier),
            114 => Ok(Self::DhcpCaptivePortal),
            255 => Ok(Self::End),
            108 => Ok(Self::UnassignedOrRemoved(value)),
            _ => Err(OptionTagError::InvalidTag(value)),
        }
    }
}

impl From<OptionTag> for u8 {
    fn from(value: OptionTag) -> Self {
        match value {
            OptionTag::Pad => 0,
            OptionTag::SubnetMask => 1,
            OptionTag::TimeOffset => 2,
            OptionTag::Router => 3,
            OptionTag::TimeServer => 4,
            OptionTag::NameServer => 5,
            OptionTag::DomainNameServer => 6,
            OptionTag::LogServer => 7,
            OptionTag::CookieServer => 8,
            OptionTag::LprServer => 9,
            OptionTag::ImpressServer => 10,
            OptionTag::ResourceLocationServer => 11,
            OptionTag::HostName => 12,
            OptionTag::BootFileSize => 13,
            OptionTag::MeritDumpFile => 14,
            OptionTag::DomainName => 15,
            OptionTag::SwapServer => 16,
            OptionTag::RootPath => 17,
            OptionTag::ExtensionsPath => 18,
            OptionTag::IpForwarding => 19,
            OptionTag::NonLocalSourceRouting => 20,
            OptionTag::PolicyFilter => 21,
            OptionTag::MaxDatagramReassemblySize => 22,
            OptionTag::DefaultIpTtl => 23,
            OptionTag::PathMtuAgingTimeout => 24,
            OptionTag::PathMtuPlateauTable => 25,
            OptionTag::InterfaceMtu => 26,
            OptionTag::AllSubnetsLocal => 27,
            OptionTag::BroadcastAddr => 28,
            OptionTag::PerformMaskDiscovery => 29,
            OptionTag::MaskSupplier => 30,
            OptionTag::PerformRouterDiscovery => 31,
            OptionTag::RouterSolicitationAddr => 32,
            OptionTag::StaticRoute => 33,
            OptionTag::TrailerEncapsulation => 34,
            OptionTag::ArpCacheTimeout => 35,
            OptionTag::EthernetEncapsulation => 36,
            OptionTag::TcpDefaultTtl => 37,
            OptionTag::TcpKeepaliveInterval => 38,
            OptionTag::TcpKeepaliveGarbage => 39,
            OptionTag::NetworkInformationServiceDomain => 40,
            OptionTag::NetworkInformationServers => 41,
            OptionTag::NetworkTimeProtocolServers => 42,
            OptionTag::VendorSpecificInformation => 43,
            OptionTag::NetbiosNameServer => 44,
            OptionTag::NetbiosDatagramDistributionServer => 45,
            OptionTag::NetbiosNodeType => 46,
            OptionTag::NetbiosScope => 47,
            OptionTag::XWindowSystemFontServer => 48,
            OptionTag::XWindowSystemDisplayManager => 49,
            OptionTag::RequestedIpAddr => 50,
            OptionTag::IpAddrLeaseTime => 51,
            OptionTag::OptionOverload => 52,
            OptionTag::DhcpMessageType => 53,
            OptionTag::ServerIdentifier => 54,
            OptionTag::ParameterRequestList => 55,
            OptionTag::Message => 56,
            OptionTag::MaxDhcpMessageSize => 57,
            OptionTag::RenewalT1Time => 58,
            OptionTag::RebindingT2Time => 59,
            OptionTag::ClassIdentifier => 60,
            OptionTag::ClientIdentifier => 61,
            OptionTag::DhcpCaptivePortal => 114,
            OptionTag::End => 255,
            OptionTag::UnassignedOrRemoved(v) => v,
        }
    }
}

impl From<&OptionTag> for u8 {
    fn from(value: &OptionTag) -> Self {
        Self::from(value.clone())
    }
}

impl Readable for OptionTag {
    type Error = OptionTagError;

    fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
        Self::try_from(buf.pop()?)
    }
}

impl Writeable for OptionTag {
    type Error = OptionTagError;

    fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
        buf.push(u8::from(self));
        Ok(1)
    }
}
