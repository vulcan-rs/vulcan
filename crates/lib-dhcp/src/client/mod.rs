use std::{
    net::{Ipv4Addr, SocketAddr},
    time::{self, Duration},
};

use binbuf::prelude::*;
use network_interface::{Error as InterfaceError, NetworkInterface, NetworkInterfaceConfig};
use rand::{self, Rng};
use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    time::{sleep, timeout},
};

use crate::{
    client::state::{ClientStateMachine, DhcpState},
    types::{
        options::{DhcpMessageType, ParameterRequestList},
        DhcpOption, HardwareAddr, Message, MessageError, OptionData, OptionTag,
    },
    utils, TimeoutResult, DHCP_MINIMUM_LEGAL_MAX_MESSAGE_SIZE, DHCP_SERVER_PORT,
};

mod error;
mod state;

pub use error::ClientError;

pub struct ClientBuilder {
    /// Duration before the binding process of the socket times out.
    bind_timeout: time::Duration,

    /// Duration before the read process of DHCP answers times out.
    read_timeout: time::Duration,

    /// Duration before the write process of DHCP requests times out.
    write_timeout: time::Duration,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            bind_timeout: time::Duration::from_secs(2),
            read_timeout: time::Duration::from_secs(2),
            write_timeout: time::Duration::from_secs(2),
        }
    }
}

impl ClientBuilder {
    pub fn build(&self) -> Client {
        Client {
            interface: NetworkInterface::new_afinet("eth0", Ipv4Addr::UNSPECIFIED, None, None, 1),
            hardware_address: HardwareAddr::default(),
            offered_ip_address: Ipv4Addr::UNSPECIFIED,
            write_timeout: self.write_timeout,
            dhcp_state: DhcpState::default(),
            bind_timeout: self.bind_timeout,
            read_timeout: self.read_timeout,
            transaction_id: rand::random(),
            dhcp_server_ip_address: None,
            offered_time: 0,
        }
    }

    pub fn with_bind_timeout(&mut self, bind_timeout: time::Duration) {
        self.bind_timeout = bind_timeout
    }
}

pub struct Client {
    /// Duration before the binding process of the socket times out.
    bind_timeout: time::Duration,

    /// Duration before the read process of DHCP answers times out.
    read_timeout: time::Duration,

    /// Duration before the write process of DHCP requests times out.
    write_timeout: time::Duration,

    /// Selected network interface
    interface: NetworkInterface,

    /// Hardware (MAC) address of the selected network interface
    hardware_address: HardwareAddr,

    // CLIENT STATE FIELDS
    /// Current transaction id (xid)
    transaction_id: u32,

    /// Optional DHCP server IP address. This address is only set when the
    /// client already received a DHCP message.
    dhcp_server_ip_address: Option<Ipv4Addr>,

    /// Optional offered IP address. This address is only set when the DHCP
    /// server send back a DHCPOFFER message, which contains the IP address
    /// in the `yiaddr` field.
    offered_ip_address: Ipv4Addr,

    offered_time: u32,

    /// DHCP state
    dhcp_state: DhcpState,
}

impl Client {
    /// Create a new DHCP [`Client`] with default values.
    pub fn new() -> Self {
        Self::builder().build()
    }

    /// Create a new [`ClientBuilder`] to declaratively build a [`Client`].
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Run the client as a daemon
    #[tokio::main]
    pub async fn run(&mut self) -> Result<(), ClientError> {
        // Get network interfaces. We need to select the proper one, to use
        // the correct hardware / mac address in DHCP messages.
        let interface = select_network_interface()?;
        self.interface = interface.ok_or(ClientError::NoInterfaceFound)?;

        // Extract the hardware (MAC) address from the interface
        self.hardware_address = HardwareAddr::try_from(
            self.interface
                .mac_addr
                .clone()
                .unwrap_or("00:00:00:00:00:00".into()),
        )?;

        // Create UDP socket with a bind timeout
        let socket = create_sock_with_timeout("0.0.0.0:68", self.bind_timeout).await?;
        socket.set_broadcast(true)?;

        loop {
            // We now use a state machine to keep track of the client state.
            // This is described in 4.4: https://www.rfc-editor.org/rfc/rfc2131#section-4.4
            match self.dhcp_state {
                DhcpState::Init => {
                    println!("Entering state INIT");
                    // Wait a random amount between one and ten seconds
                    let wait_duration = Duration::from_secs(rand::thread_rng().gen_range(1..=10));
                    println!(
                        "Waiting for {:?} to send DHCPDISCOVER message",
                        wait_duration
                    );
                    sleep(wait_duration).await;

                    // Send DHCPDISCOVER message
                    println!("Sending DHCPDISCOVER message");
                    let discover_message = self.make_discover_message()?;
                    self.send_message(discover_message, &socket).await?;

                    // Transition to SELECTING
                    self.transition_to(DhcpState::Selecting)?;
                }
                DhcpState::InitReboot => todo!(),
                DhcpState::Selecting => {
                    println!("Entering state SELECTING");
                    // Collect replies (DHCPOFFER)
                    // TODO (Techassi): Scale the timeout duration over time
                    let (message, addr) =
                        match utils::timeout(Duration::from_secs(2), self.recv_message(&socket))
                            .await
                        {
                            TimeoutResult::Timeout => {
                                self.transition_to(DhcpState::Init)?;
                                continue;
                            }
                            TimeoutResult::Error(err) => return Err(err),
                            TimeoutResult::Ok(result) => match result {
                                Some(result) => result,
                                None => continue,
                            },
                        };

                    // Check if the transaction ID matches
                    if self.transaction_id != message.header.xid {
                        println!(
                            "Received response with wrong transaction ID: {} (yours: {})",
                            message.header.xid, self.transaction_id
                        );
                        continue;
                    }

                    // Check if the DHCP message type is correct
                    match message.get_message_type() {
                        Some(ty) => {
                            if *ty != DhcpMessageType::Offer {
                                continue;
                            }
                        }
                        None => {
                            println!("Received response with no DHCP message type option set");
                            continue;
                        }
                    }

                    // Select offer
                    // Set destination server IP address
                    if let Some(option) = message.get_option(OptionTag::ServerIdentifier) {
                        match option.data() {
                            OptionData::ServerIdentifier(ip) => {
                                self.dhcp_server_ip_address = Some(*ip)
                            }
                            _ => {}
                        }
                    }

                    // Set offered IP address lease time
                    if let Some(option) = message.get_option(OptionTag::IpAddrLeaseTime) {
                        match option.data() {
                            OptionData::IpAddrLeaseTime(time) => self.offered_time = *time,
                            _ => {}
                        }
                    }

                    // Set offered IP address
                    self.offered_ip_address = message.yiaddr;

                    // Send DHCPREQUEST message
                    println!("Sending DHCPREQUEST message");
                    let request_message = self.make_request_message()?;
                    self.send_message(request_message, &socket).await?;

                    // Transition to REQUESTING
                    self.transition_to(DhcpState::Requesting)?;
                }
                DhcpState::Rebooting => todo!(),
                DhcpState::Requesting => {
                    println!("Entering REQUESTING");
                    // Discard other DHCPOFFER

                    // We should get a DHCPACK or DHCPNAK message
                    // TODO (Techassi): Scale the timeout duration over time
                    let (message, addr) =
                        match utils::timeout(Duration::from_secs(2), self.recv_message(&socket))
                            .await
                        {
                            TimeoutResult::Timeout => {
                                self.transition_to(DhcpState::Init)?;
                                continue;
                            }
                            TimeoutResult::Error(err) => return Err(err),
                            TimeoutResult::Ok(result) => match result {
                                Some(result) => result,
                                None => continue,
                            },
                        };

                    // TODO (Techassi): We should introduce a timer which ticks everytime we encounter this code path to
                    // not get stuck in this state
                    match message.get_message_type() {
                        Some(ty) => match ty {
                            DhcpMessageType::Nak => {
                                self.transition_to(DhcpState::Init)?;
                                continue;
                            }
                            DhcpMessageType::Ack => {}
                            _ => continue,
                        },
                        None => continue,
                    }

                    // Set lease, T1 and T2 timers (DHCPACK)
                    println!("Setting lease, T1 and T2 timers");

                    // Send DHCPACK message
                    println!("Sending DHCPACK message");
                    let ack_message = self.make_ack_message()?;
                    self.send_message(ack_message, &socket).await?;

                    // Transition to BOUND
                    self.transition_to(DhcpState::Bound)?;
                }
                DhcpState::Rebinding => {
                    // Set lease, T1 and T2 timers (DHCPACK)
                    // Transition to BOUND

                    // Lease expired (DHCPNAK), return to INIT
                }
                DhcpState::Bound => {
                    println!("Entering BOUND")
                    // Remain in this state. Discard incoming
                    // DHCPOFFER, DHCPACK and DHCPNAK

                    // T1 expires, send DHCPREQUEST to leasing server

                    // Transition to RENEWING
                }
                DhcpState::Renewing => {
                    // Set lease, T1 and T2 timers (DHCPACK)

                    // DHCPNAK, return to INIT

                    // T2 expires, broadcast DHCPREQUEST
                    // Transition to REBINDING
                }
            }
        }
    }

    fn destination_addr(&self) -> Ipv4Addr {
        if self.dhcp_server_ip_address.is_some() {
            return self.dhcp_server_ip_address.unwrap();
        }

        Ipv4Addr::BROADCAST
    }

    /// Receive a DHCP message. This internally runs through the following
    /// steps:
    ///
    /// 1. Wait for the UDP socket to be readable. This can produce false
    ///    positives
    /// 2. Create a buffer with the minimum legal max DHCP message size
    /// 3. Try to receive UDP datagram from the socket
    /// 4. Create ReadBuffer and parse message
    /// 5. Return optional message and SocketAddr
    ///
    /// If the function returns Ok(None), `readable` produced a false
    /// positive and we catched a `WouldBlock` error.
    async fn recv_message(
        &self,
        sock: &UdpSocket,
    ) -> Result<Option<(Message, SocketAddr)>, ClientError> {
        // First try to retreive one (if any) UDP datagrams.
        // readable can produce a false positive, which is why we need to
        // check for errors when calling try_recv_from.
        sock.readable().await?;

        // Create an empty (all 0s) buffer with the minimum legal max DHCP
        // message size
        let mut buf = vec![0u8; DHCP_MINIMUM_LEGAL_MAX_MESSAGE_SIZE.into()];

        let (buf, addr) = match sock.try_recv_from(&mut buf) {
            Ok((len, addr)) => (&buf[..len], addr),
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => return Ok(None),
            Err(e) => {
                return Err(e.into());
            }
        };

        let mut buf = ReadBuffer::new(buf);
        Ok(Some((Message::read_be(&mut buf)?, addr)))
    }

    /// Send a DHCP message / packet with the default timeouts to `dest_addr`
    /// by binding to `bind_addr`. The bind address is usually `0.0.0.0:68`.
    /// The default timeouts can be adjusted by using [`Client::builder`]
    async fn send_message(&self, message: Message, socket: &UdpSocket) -> Result<(), ClientError> {
        // Choose a destion IP address. This is either the broadcast address
        // or the DHCP server address.
        let destination_addr = self.destination_addr();

        // Create the write buffer
        let mut buf = WriteBuffer::new();
        message.write_be(&mut buf)?;

        // Off to the wire the bytes go
        socket
            .send_to(buf.bytes(), (destination_addr, DHCP_SERVER_PORT))
            .await?;

        Ok(())
    }

    /// This creates a new DHCPDISCOVER message with the values described in
    /// RFC 2131 Section 4.4.1
    fn make_discover_message(&mut self) -> Result<Message, MessageError> {
        // The client sets 'ciaddr' to 0x00000000. This is already done in
        // Message::new() (Default value).
        let mut message = Message::new_with_xid(self.transaction_id);

        // Set DHCP options
        // Set DHCP message type option
        message.add_option(DhcpOption::new(
            OptionTag::DhcpMessageType,
            OptionData::DhcpMessageType(DhcpMessageType::Discover),
        ))?;

        // Maximum DHCP message size
        // TODO (Techassi): Don't hardcode this
        message.add_option(DhcpOption::new(
            OptionTag::MaxDhcpMessageSize,
            OptionData::MaxDhcpMessageSize(1500),
        ))?;

        // Set DHCP hostname option
        message.add_option(DhcpOption::new(
            OptionTag::HostName,
            OptionData::HostName("hardcoded".to_string()),
        ))?;

        // The client MAY request specific parameters by including the
        // 'parameter request list' option.
        message.add_option(self.default_request_parameter_list())?;
        message.add_option(DhcpOption::new(OptionTag::End, OptionData::End))?;

        // The client MAY suggest a network address and/or lease time by
        // including the 'requested IP address' and 'IP address lease time'
        // options.

        // The client MUST include its hardware address in the 'chaddr' field,
        // if necessary for delivery of DHCP reply messages.
        message.set_hardware_address(self.hardware_address.clone());

        // The client MAY include a different unique identifier in the 'client
        // identifier' option, as discussed in section 4.2.

        Ok(message)
    }

    fn make_request_message(&self) -> Result<Message, MessageError> {
        let mut message = Message::new_with_xid(self.transaction_id);

        // Set DHCP message type option
        message.add_option(DhcpOption::new(
            OptionTag::DhcpMessageType,
            OptionData::DhcpMessageType(DhcpMessageType::Request),
        ))?;

        // Set maximum DHCP message size option
        // TODO (Techassi): Don't hardcode this
        message.add_option(DhcpOption::new(
            OptionTag::MaxDhcpMessageSize,
            OptionData::MaxDhcpMessageSize(1500),
        ))?;

        // Set DHCP hostname option
        message.add_option(DhcpOption::new(
            OptionTag::HostName,
            OptionData::HostName("hardcoded".to_string()),
        ))?;

        // Set DHCP server identifier option
        message.add_option(DhcpOption::new(
            OptionTag::ServerIdentifier,
            OptionData::ServerIdentifier(self.dhcp_server_ip_address.unwrap()),
        ))?;

        // Set DHCP requested IP address option
        message.add_option(DhcpOption::new(
            OptionTag::RequestedIpAddr,
            OptionData::RequestedIpAddr(self.offered_ip_address),
        ))?;

        // Set DHCP IP address lease time
        message.add_option(DhcpOption::new(
            OptionTag::IpAddrLeaseTime,
            OptionData::IpAddrLeaseTime(self.offered_time),
        ))?;

        message.add_option(self.default_request_parameter_list())?;
        message.add_option(DhcpOption::new(OptionTag::End, OptionData::End))?;

        message.set_hardware_address(self.hardware_address.clone());

        Ok(message)
    }

    fn make_ack_message(&self) -> Result<Message, ClientError> {
        Ok(Message::default())
    }

    fn default_request_parameter_list(&self) -> DhcpOption {
        DhcpOption::new(
            OptionTag::ParameterRequestList,
            OptionData::ParameterRequestList(ParameterRequestList::new(vec![
                OptionTag::Router,
                OptionTag::DomainNameServer,
                OptionTag::RenewalT1Time,
                OptionTag::RebindingT2Time,
            ])),
        )
    }
}

fn select_network_interface() -> Result<Option<NetworkInterface>, InterfaceError> {
    let interfaces = NetworkInterface::show()?;
    for interface in interfaces {
        // Filter out interfaces like loopback (lo) and wireguard (wgX)
        if interface.name.starts_with("lo") || interface.name.starts_with("wg") {
            continue;
        }

        // TODO (Techassi): This should also filter out null addresses
        if interface.mac_addr.is_none() {
            continue;
        }

        // Filter out interfaces with IPv6 addresses, as this DHCP
        // implementation is aimed at IPv4
        if interface.addr.filter(|a| a.ip().is_ipv6()).is_some() {
            continue;
        }

        // Hopefully the right interface
        return Ok(Some(interface));
    }

    Ok(None)
}

// TODO (Techassi): Don't return a client error here, but instead a more
// generic error like ProtoError or io::Error
async fn create_sock_with_timeout<A>(
    bind_addr: A,
    bind_timeout: time::Duration,
) -> Result<UdpSocket, ClientError>
where
    A: ToSocketAddrs,
{
    match timeout(bind_timeout, UdpSocket::bind(bind_addr)).await {
        Ok(sock_result) => match sock_result {
            Ok(sock) => Ok(sock),
            Err(err) => return Err(ClientError::IO(err)),
        },
        Err(_) => return Err(ClientError::BindTimeout(bind_timeout)),
    }
}
