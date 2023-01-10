use binbuf::prelude::*;

pub enum OptionTag {
    /// #### Pad Option
    ///
    /// The pad option can be used to cause subsequent fields to align on word
    /// boundaries.
    ///
    /// See [3.1. Pad Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.1)
    Pad,

    /// #### End Option
    ///
    /// The end option marks the end of valid information in the vendor field.
    /// Subsequent octets should be filled with pad options.
    ///
    /// See [3.2. End Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.2)
    End,

    /// #### Subnet Mask
    ///
    /// The subnet mask option specifies the client's subnet mask as per RFC
    /// 950. If both the subnet mask and the router option are specified in
    /// a DHCP reply, the subnet mask option MUST be first.
    ///
    /// See
    /// - [3.3. Subnet Mask](https://datatracker.ietf.org/doc/html/rfc1533#section-3.3)
    /// - [RFC 950](https://datatracker.ietf.org/doc/html/rfc950)
    SubnetMask,

    /// #### Time Offset
    ///
    /// The time offset field specifies the offset of the client's subnet in
    /// seconds from Coordinated Universal Time (UTC). The offset is
    /// expressed as a signed 32-bit integer.
    ///
    /// See [3.4. Time Offset](https://datatracker.ietf.org/doc/html/rfc1533#section-3.4)
    TimeOffset,

    /// Router Option
    ///
    /// The router option specifies a list of IP addresses for routers on the
    /// client's subnet.  Routers SHOULD be listed in order of preference.
    ///
    /// See [3.5. Router Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.5)
    Router,

    /// #### Time Server Option
    ///
    /// The time server option specifies a list of RFC 868 time servers
    /// available to the client. Servers SHOULD be listed in order of
    /// preference.
    ///
    /// See
    /// - [3.6. Time Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.6)
    /// - [RFC 868](https://datatracker.ietf.org/doc/html/rfc868)
    TimeServer,

    /// #### Name Server Option
    ///
    /// The name server option specifies a list of IEN 116 name servers
    /// available to the client. Servers SHOULD be listed in order of
    /// preference.
    ///
    /// See [3.7. Name Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.7)
    NameServer,

    /// #### Domain Name Server Option
    ///
    /// The domain name server option specifies a list of Domain Name System
    /// (STD 13, RFC 1035) name servers available to the client. Servers
    /// SHOULD be listed in order of preference.
    ///
    /// See
    /// - [3.8. Domain Name Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.8)
    /// - [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035)
    DomainNameServer,

    /// #### Log Server Option
    ///
    /// The log server option specifies a list of MIT-LCS UDP log servers
    /// available to the client. Servers SHOULD be listed in order of
    /// preference.
    ///
    /// See [3.9. Log Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.9)
    LogServer,

    /// #### Cookie Server Option
    ///
    /// The cookie server option specifies a list of RFC 865 cookie servers
    /// available to the client. Servers SHOULD be listed in order of
    /// preference.
    ///
    /// See
    /// - [3.10. Cookie Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.10)
    /// - [RFC 865](https://datatracker.ietf.org/doc/html/rfc865)
    CookieServer,

    /// #### LPR Server Option
    ///
    /// The LPR server option specifies a list of RFC 1179 line printer servers
    /// available to the client. Servers SHOULD be listed in order of
    /// preference.
    ///
    /// See
    /// - [3.11. LPR Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.11)
    /// - [RFC 1179](https://datatracker.ietf.org/doc/html/rfc1179)
    LprServer,

    /// #### Impress Server Option
    ///
    /// The Impress server option specifies a list of Imagen Impress servers
    /// available to the client. Servers SHOULD be listed in order of
    /// preference.
    ///
    /// See [3.12. Impress Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.12)
    ImpressServer,

    /// #### Resource Location Server Option
    ///
    /// This option specifies a list of RFC 887 Resource Location servers
    /// available to the client. Servers SHOULD be listed in order of
    /// preference.
    ///
    /// See
    /// - [3.13. Resource Location Server Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.13)
    /// - [RFC 887](https://datatracker.ietf.org/doc/html/rfc887)
    ResourceLocationServer,

    /// #### Host Name Option
    ///
    /// This option specifies the name of the client. The name may or may not
    /// be qualified with the local domain name (see [section 3.17][1] for the
    /// preferred way to retrieve the domain name). See RFC 1035 for character
    /// set restrictions.
    ///
    /// See
    /// - [3.14. Host Name Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.14)
    /// - [RFC 1035](https://datatracker.ietf.org/doc/html/rfc1035)
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-3.17
    HostName,

    /// #### Boot File Size Option
    ///
    /// This option specifies the length in 512-octet blocks of the default
    /// boot image for the client. The file length is specified as an unsigned
    /// 16-bit integer.
    ///
    /// See [3.15. Boot File Size Option](https://datatracker.ietf.org/doc/html/rfc1533#section-3.15)
    BootFileSize,

    /// #### Merit Dump File
    ///
    /// This option specifies the path-name of a file to which the client's
    /// core image should be dumped in the event the client crashes. The path
    /// is formatted as a character string consisting of characters from the
    /// NVT ASCII character set.
    ///
    /// See [3.16. Merit Dump File](https://datatracker.ietf.org/doc/html/rfc1533#section-3.16)
    MeritDumpFile,

    /// #### Domain Name
    ///
    /// This option specifies the domain name that client should use when
    /// resolving hostnames via the Domain Name System.
    ///
    /// See [3.17. Domain Name](https://datatracker.ietf.org/doc/html/rfc1533#section-3.17)
    DomainName,

    /// #### Swap Server
    ///
    /// This specifies the IP address of the client's swap server.
    ///
    /// See [3.18. Swap Server](https://datatracker.ietf.org/doc/html/rfc1533#section-3.18)
    SwapServer,

    /// #### Root Path
    ///
    /// This option specifies the path-name that contains the client's root
    /// disk. The path is formatted as a character string consisting of
    /// characters from the NVT ASCII character set.
    ///
    /// See [3.19. Root Path](https://datatracker.ietf.org/doc/html/rfc1533#section-3.19)
    RootPath,

    /// #### Extensions Path
    ///
    /// A string to specify a file, retrievable via TFTP, which contains
    /// information which can be interpreted in the same way as the 64-octet
    /// vendor-extension field within the BOOTP response, with the following
    /// exceptions:
    ///
    /// - the length of the file is unconstrained;
    /// - all references to Tag 18 (i.e., instances of the BOOTP Extensions
    ///   Path field) within the file are ignored.
    ///
    /// See [3.20. Extensions Path](https://datatracker.ietf.org/doc/html/rfc1533#section-3.20)
    ExtensionsPath,

    /// #### IP Forwarding Enable/Disable Option
    ///
    /// This option specifies whether the client should configure its IP layer
    /// for packet forwarding.  A value of 0 means disable IP forwarding, and
    /// a value of 1 means enable IP forwarding.
    ///
    /// See [4.1. IP Forwarding Enable/Disable Option](https://datatracker.ietf.org/doc/html/rfc1533#section-4.1)
    IpForwarding,

    /// #### Non-Local Source Routing Enable/Disable Option
    ///
    /// This option specifies whether the client should configure its IP layer
    /// to allow forwarding of datagrams with non-local source routes (see
    /// Section 3.3.5 of [[4][1]] for a discussion of this topic). A value of
    /// 0 means disallow forwarding of such datagrams, and a value of 1 means
    /// allow forwarding.
    ///
    /// See [4.2. Non-Local Source Routing Enable/Disable Option][2]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#ref-4
    /// [2]: https://datatracker.ietf.org/doc/html/rfc1533#section-4.2
    NonLocalSourceRouting,

    /// #### Policy Filter Option
    ///
    /// This option specifies policy filters for non-local source routing. The
    /// filters consist of a list of IP addresses and masks which specify
    /// destination/mask pairs with which to filter incoming source routes.
    ///
    /// Any source routed datagram whose next-hop address does not match one of
    /// the filters should be discarded by the client.
    ///
    /// See [[4][1]] for further information.
    ///
    /// See [4.3. Policy Filter Option][2]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#ref-4
    /// [2]: https://datatracker.ietf.org/doc/html/rfc1533#section-4.3
    PolicyFilter,

    /// #### Maximum Datagram Reassembly Size
    ///
    /// This option specifies the maximum size datagram that the client should
    /// be prepared to reassemble.  The size is specified as a 16-bit unsigned
    /// integer. The minimum value legal value is 576.
    ///
    /// See [4.4. Maximum Datagram Reassembly Size][1]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-4.4
    MaxDatagramReassemblySize,

    /// #### Default IP Time-to-live
    ///
    /// This option specifies the default time-to-live that the client should
    /// use on outgoing datagrams.  The TTL is specified as an octet with a
    /// value between 1 and 255.
    ///
    /// See [4.5. Default IP Time-to-live][1]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-4.5
    DefaultIpTtl,

    /// #### Path MTU Aging Timeout Option
    ///
    /// This option specifies the timeout (in seconds) to use when aging Path
    /// MTU values discovered by the mechanism defined in [RFC 1191][1]
    /// [[12][2]]. The timeout is specified as a 32-bit unsigned integer.
    ///
    /// See [4.6. Path MTU Aging Timeout Option][3]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1191
    /// [2]: https://datatracker.ietf.org/doc/html/rfc1533#ref-12
    /// [3]: https://datatracker.ietf.org/doc/html/rfc1533#section-4.6
    PathMtuAgingTimeout,

    /// #### Path MTU Plateau Table Option
    ///
    /// This option specifies a table of MTU sizes to use when performing Path
    /// MTU Discovery as defined in [RFC 1191][1]. The table is formatted as a
    /// list of 16-bit unsigned integers, ordered from smallest to largest. The
    /// minimum MTU value cannot be smaller than 68.
    ///
    /// See [4.7. Path MTU Plateau Table Option][²]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1191
    /// [2]: https://datatracker.ietf.org/doc/html/rfc1533#section-4.7
    PathMtuPlateauTable,

    /// #### Interface MTU Option
    ///
    /// This option specifies the MTU to use on this interface. The MTU is
    /// specified as a 16-bit unsigned integer. The minimum legal value for the
    /// MTU is 68.
    ///
    /// See [5.1. Interface MTU Option][1]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-5.1
    InterfaceMtu,

    /// All Subnets are Local Option
    ///
    /// This option specifies whether or not the client may assume that all
    /// subnets of the IP network to which the client is connected use the same
    /// MTU as the subnet of that network to which the client is directly
    /// connected. A value of 1 indicates that all subnets share the same MTU.
    /// A value of 0 means that the client should assume that some subnets of
    /// the directly connected network may have smaller MTUs.
    ///
    /// See [5.2. All Subnets are Local Option][1]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-5.2
    AllSubnetsLocal,

    /// #### Broadcast Address Option
    ///
    /// This option specifies the broadcast address in use on the client's
    /// subnet. Legal values for broadcast addresses are specified in section
    /// 3.2.1.3 of [[4][1]].
    ///
    /// See [5.3. Broadcast Address Option][2]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#ref-4
    /// [2]: https://datatracker.ietf.org/doc/html/rfc1533#section-5.3
    BroadcastAddr,

    /// #### Perform Mask Discovery Option
    ///
    /// This option specifies whether or not the client should perform subnet
    /// mask discovery using ICMP. A value of 0 indicates that the client
    /// should not perform mask discovery. A value of 1 means that the client
    /// should perform mask discovery.
    ///
    /// See [5.4. Perform Mask Discovery Option][1]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-5.4
    PerformMaskDiscovery,

    /// #### Mask Supplier Option
    ///
    /// This option specifies whether or not the client should respond to
    /// subnet mask requests using ICMP. A value of 0 indicates that the client
    /// should not respond. A value of 1 means that the client should respond.
    ///
    /// See [5.5. Mask Supplier Option][1]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-5.5
    MaskSupplier,

    /// #### Perform Router Discovery Option
    ///
    /// This option specifies whether or not the client should solicit routers
    /// using the Router Discovery mechanism defined in [RFC 1256][1]
    /// [[13][2]]. A value of 0 indicates that the client should not perform
    /// router discovery. A value of 1 means that the client should perform
    /// router discovery.
    ///
    /// See [5.6. Perform Router Discovery Option][3]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1256
    /// [2]: https://datatracker.ietf.org/doc/html/rfc1533#ref-13
    /// [3]: https://datatracker.ietf.org/doc/html/rfc1533#section-5.6
    PerformRouterDiscovery,

    /// #### Router Solicitation Address Option
    ///
    /// This option specifies the address to which the client should transmit
    /// router solicitation requests.
    ///
    /// See [5.7. Router Solicitation Address Option][1]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-5.7
    RouterSolicitationAddr,

    /// #### Static Route Option
    ///
    /// This option specifies a list of static routes that the client should
    /// install in its routing cache. If multiple routes to the same
    /// destination are specified, they are listed in descending order of
    /// priority.
    ///
    /// The routes consist of a list of IP address pairs. The first address is
    /// the destination address, and the second address is the router for the
    /// destination.
    ///
    /// The default route (0.0.0.0) is an illegal destination for a static
    /// route. See [section 3.5][1] for information about the router option.
    ///
    /// See [5.8. Static Route Option][2]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-3.5
    /// [2]: https://datatracker.ietf.org/doc/html/rfc1533#section-5.8
    StaticRoute,

    /// #### Trailer Encapsulation Option
    ///
    /// This option specifies whether or not the client should negotiate the
    /// use of trailers ([RFC 893][1] [[14][2]]) when using the ARP protocol.
    /// A value of 0 indicates that the client should not attempt to use
    /// trailers. A value of 1 means that the client should attempt to use
    /// trailers.
    ///
    /// See [6.1. Trailer Encapsulation Option][3]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc893
    /// [2]: https://datatracker.ietf.org/doc/html/rfc1533#ref-14
    /// [3]: https://datatracker.ietf.org/doc/html/rfc1533#section-6.1
    TrailerEncapsulation,

    /// #### ARP Cache Timeout Option
    ///
    /// This option specifies the timeout in seconds for ARP cache entries. The
    /// time is specified as a 32-bit unsigned integer.
    ///
    /// See [6.2. ARP Cache Timeout Option][1]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-6.2
    ArpCacheTimeout,

    /// #### Ethernet Encapsulation Option
    ///
    /// This option specifies whether or not the client should use Ethernet
    /// Version 2 ([RFC 894][1] [[15][2]]) or IEEE 802.3 ([RFC 1042][3]
    /// [[16][4]]) encapsulation if the interface is an Ethernet. A value of
    /// 0 indicates that the client should use [RFC 894][1] encapsulation.
    /// A value of 1 means that the client should use [RFC 1042][3]
    /// encapsulation.
    ///
    /// See [6.3. Ethernet Encapsulation Option][5]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc894
    /// [2]: https://datatracker.ietf.org/doc/html/rfc1533#ref-15
    /// [3]: https://datatracker.ietf.org/doc/html/rfc1042
    /// [4]: https://datatracker.ietf.org/doc/html/rfc1533#ref-16
    /// [5]: https://datatracker.ietf.org/doc/html/rfc1533#section-6.3
    EthernetEncapsulation,

    /// #### TCP Default TTL Option
    ///
    /// This option specifies the default TTL that the client should use when
    /// sending TCP segments. The value is represented as an 8-bit unsigned
    /// integer. The minimum value is 1.
    ///
    /// See [7.1. TCP Default TTL Option][1]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-7.1
    TcpDefaultTtl,

    /// #### TCP Keepalive Interval Option
    ///
    /// This option specifies the interval (in seconds) that the client TCP
    /// should wait before sending a keepalive message on a TCP connection.
    /// The time is specified as a 32-bit unsigned integer. A value of zero
    /// indicates that the client should not generate keepalive messages on
    /// connections unless specifically requested by an application.
    ///
    /// See [7.2. TCP Keepalive Interval Option][1]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-7.2
    TcpKeepaliveInterval,

    /// #### TCP Keepalive Garbage Option
    ///
    /// This option specifies the whether or not the client should send TCP
    /// keepalive messages with a octet of garbage for compatibility with older
    /// implementations. A value of 0 indicates that a garbage octet should not
    /// be sent. A value of 1 indicates that a garbage octet should be sent.
    ///
    /// See [7.3. TCP Keepalive Garbage Option][1]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-7.3
    TcpKeepaliveGarbage,

    /// #### Network Information Service Domain Option
    ///
    /// This option specifies the name of the client's NIS [[17][1]] domain.
    /// The domain is formatted as a character string consisting of characters
    /// from the NVT ASCII character set.
    ///
    /// See [8.1. Network Information Service Domain Option][2]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#ref-17
    /// [2]: https://datatracker.ietf.org/doc/html/rfc1533#section-8.1
    NetworkInformationServiceDomain,

    /// ### Network Information Servers Option
    ///
    /// This option specifies a list of IP addresses indicating NIS servers
    /// available to the client. Servers SHOULD be listed in order of
    /// preference.
    ///
    /// See [8.2. Network Information Servers Option][1]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-8.2
    NetworkInformationServers,

    /// #### Network Time Protocol Servers Option
    ///
    /// This option specifies a list of IP addresses indicating NTP [[18][1]]
    /// servers available to the client. Servers SHOULD be listed in order of
    /// preference.
    ///
    /// See [8.3. Network Time Protocol Servers Option][2]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#ref-18
    /// [2]: https://datatracker.ietf.org/doc/html/rfc1533#section-8.3
    NetworkTimeProtocolServers,

    /// #### Vendor Specific Information
    ///
    /// This option is used by clients and servers to exchange vendor-specific
    /// information. The information is an opaque object of n octets,
    /// presumably interpreted by vendor-specific code on the clients and
    /// servers. The definition of this information is vendor specific. The
    /// vendor is indicated in the class-identifier option. Servers not
    /// equipped to interpret the vendor-specific information sent by a client
    /// MUST ignore it (although it may be reported). Clients which do not
    /// receive desired vendor-specific information SHOULD make anattempt to
    /// operate without it, although they may do so (and announce they are
    /// doing so) in a degraded mode.
    ///
    /// If a vendor potentially encodes more than one item of information in
    /// this option, then the vendor SHOULD encode the option using
    /// "Encapsulated vendor-specific options" as described below:
    ///
    /// The Encapsulated vendor-specific options field SHOULD be encoded as
    /// a sequence of code/length/value fields of identical syntax to the DHCP
    /// options field with the following exceptions:
    ///
    /// 1. There SHOULD NOT be a "magic cookie" field in the encapsulated
    ///    vendor-specific extensions field.
    /// 2. Codes other than 0 or 255 MAY be redefined by the vendor within
    ///    the encapsulated vendor-specific extensions field, but SHOULD
    ///    conform to the tag-length-value syntax defined in [section 2][1].
    /// 3. Code 255 (END), if present, signifies the end of the encapsulated
    ///    vendor extensions, not the end of the vendor extensions field. If no
    ///    code 255 is present, then the end of the enclosing vendor-specific
    ///    information field is taken as the end of the encapsulated vendor-
    ///    specific extensions field.
    ///
    /// See [8.4. Vendor Specific Information][2]
    ///
    /// [1]: https://datatracker.ietf.org/doc/html/rfc1533#section-2
    /// [2]: https://datatracker.ietf.org/doc/html/rfc1533#section-8.4
    VendorSpecificInformation,

    /// #### NetBIOS over TCP/IP Name Server Option
    NetbiosNameServer,
    NetbiosDatagramDistributionServer,
    NetbiosNodeType,
    NetbiosScope,
    XWindowSystemFontServer,
    XWindowSystemDisplayManager,
    RequestedIpAddr,
    IpAddrLeaseTime,
    OptionOverload,
    DhcpMessageType,
    ServerIdentifier,
    ParameterRequestList,
    Message,
    MaxDhcpMessageSize,
    RenewalT1Time,
    RebindingT2Time,
    ClassIdentifier,
    ClientIdentifier,
}

impl Readable for OptionTag {
    type Error = BufferError;

    fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
        let ty = buf.pop()?;

        match ty {
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
            255 => Ok(Self::End),
            _ => Err(BufferError::InvalidData), // TODO (Techassi): Add custom error
        }
    }
}
