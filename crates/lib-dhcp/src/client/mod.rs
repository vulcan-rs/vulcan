use std::{
    net::{Ipv4Addr, SocketAddr},
    time::{self, Duration},
};

use binbuf::prelude::*;
use network_interface::NetworkInterface;
use rand::{self, Rng};
use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    time::{sleep, timeout},
};

use crate::{
    client::state::{ClientState, DhcpState, DhcpStateMachine},
    types::{
        options::{DhcpMessageType, ParameterRequestList},
        DhcpOption, HardwareAddr, Message, MessageError, OptionData, OptionTag,
    },
    utils, TimeoutResult, DHCP_MINIMUM_LEGAL_MAX_MESSAGE_SIZE, DHCP_SERVER_PORT,
};

mod cmd;
mod error;
mod state;
mod timers;

pub use error::ClientError;

pub struct ClientBuilder {
    /// Duration before the binding process of the socket times out.
    bind_timeout: time::Duration,

    /// Duration before the read process of DHCP answers times out.
    read_timeout: time::Duration,

    /// Duration before the write process of DHCP requests times out.
    write_timeout: time::Duration,

    interface: String,

    interface_fallback: bool,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            bind_timeout: time::Duration::from_secs(2),
            read_timeout: time::Duration::from_secs(2),
            write_timeout: time::Duration::from_secs(2),
            interface: String::from("eth0"),
            interface_fallback: false,
        }
    }
}

impl ClientBuilder {
    pub fn build(self) -> Result<Client, ClientError> {
        let interface =
            match utils::select_network_interface(&self.interface, self.interface_fallback)? {
                Some(ifa) => ifa,
                None => return Err(ClientError::NoInterfaceFound),
            };

        let hardware_address = match &interface.mac_addr {
            Some(mac_addr) => HardwareAddr::try_from(mac_addr)?,
            None => return Err(ClientError::NoHardwareAddressError(interface.name)),
        };

        Ok(Client {
            client_state: ClientState::default(),
            write_timeout: self.write_timeout,
            dhcp_state: DhcpState::default(),
            bind_timeout: self.bind_timeout,
            read_timeout: self.read_timeout,
            hardware_address,
            interface,
        })
    }

    pub fn with_bind_timeout(mut self, bind_timeout: time::Duration) -> Self {
        self.bind_timeout = bind_timeout;
        self
    }

    pub fn with_read_timeout(mut self, read_timeout: time::Duration) -> Self {
        self.read_timeout = read_timeout;
        self
    }

    pub fn with_write_timeout(mut self, write_timeout: time::Duration) -> Self {
        self.write_timeout = write_timeout;
        self
    }

    pub fn with_interface_name<T: Into<String>>(mut self, interface: T) -> Self {
        self.interface = interface.into();
        self
    }

    pub fn with_interface_fallback(mut self, fallback: bool) -> Self {
        self.interface_fallback = fallback;
        self
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

    /// Client state
    client_state: ClientState,

    /// DHCP state
    dhcp_state: DhcpState,
}

impl Client {
    /// Create a new DHCP [`Client`] with default values.
    pub fn new() -> Result<Self, ClientError> {
        Self::builder().build()
    }

    /// Create a new [`ClientBuilder`] to declaratively build a [`Client`].
    pub fn builder() -> ClientBuilder {
        ClientBuilder::default()
    }

    /// Run the client as a daemon
    #[tokio::main]
    pub async fn run(&mut self) -> Result<(), ClientError> {
        // Create UDP socket with a bind timeout
        let socket = create_sock_with_timeout("0.0.0.0:68", self.bind_timeout).await?;
        socket.bind_device(Some(self.interface.name.as_bytes()))?;
        socket.set_broadcast(true)?;

        // Ensure the interface is UP
        cmd::set_interface_up(&self.interface.name)?;

        loop {
            // We now use a state machine to keep track of the client state.
            // This is described in 4.4: https://www.rfc-editor.org/rfc/rfc2131#section-4.4
            match self.dhcp_state {
                DhcpState::Init => self.handle_state_init(&socket).await?,
                DhcpState::InitReboot => todo!(),
                DhcpState::Selecting => self.handle_state_selecting(&socket).await?,
                DhcpState::Rebooting => todo!(),
                DhcpState::Requesting => self.handle_state_requesting(&socket).await?,
                DhcpState::Rebinding => {
                    // Set lease, T1 and T2 timers (DHCPACK)
                    // Transition to BOUND

                    // Lease expired (DHCPNAK), return to INIT
                }
                DhcpState::Bound => self.handle_state_bound(&socket).await?,
                DhcpState::Renewing => self.handle_state_renewing(&socket).await?,
            }
        }
    }

    /// Handle the DHCP state INIT
    async fn handle_state_init(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
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
        Ok(self.transition_to(DhcpState::Selecting)?)
    }

    /// Handle the DHCP state SELECTING
    async fn handle_state_selecting(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
        println!("Entering state SELECTING");
        // Collect replies (DHCPOFFER)
        // TODO (Techassi): Scale the timeout duration over time
        let (message, _addr) =
            match utils::timeout(self.read_timeout, self.recv_message(&socket)).await {
                TimeoutResult::Timeout => {
                    self.transition_to(DhcpState::Init)?;
                    return Ok(());
                }
                TimeoutResult::Error(err) => return Err(err),
                TimeoutResult::Ok(result) => match result {
                    Some(result) => result,
                    None => return Ok(()),
                },
            };

        // Check if the transaction ID matches
        let xid = self.get_xid();
        if xid != message.header.xid {
            println!(
                "Received response with wrong transaction ID: {} (yours: {})",
                message.header.xid, xid
            );
            return Ok(());
        }

        // Check if the DHCP message type is correct
        match message.get_message_type() {
            Some(ty) => {
                if *ty != DhcpMessageType::Offer {
                    return Ok(());
                }
            }
            None => {
                println!("Received response with no DHCP message type option set");
                return Ok(());
            }
        }

        // Select offer
        // Set destination server IP address
        if let Some(option) = message.get_option(OptionTag::ServerIdentifier) {
            match option.data() {
                OptionData::ServerIdentifier(ip) => self.client_state.server_identifier = Some(*ip),
                _ => {}
            }
        }

        // Set offered IP address lease time
        if let Some(option) = message.get_option(OptionTag::IpAddrLeaseTime) {
            match option.data() {
                OptionData::IpAddrLeaseTime(time) => {
                    self.client_state.offered_lease_time = Some(*time)
                }
                _ => {}
            }
        }

        // Set offered IP address
        self.client_state.offered_ip_address = Some(message.yiaddr);

        // Send DHCPREQUEST message
        println!("Sending DHCPREQUEST message");
        let request_message = self.make_request_message()?;
        self.send_message(request_message, &socket).await?;

        // Transition to REQUESTING
        Ok(self.transition_to(DhcpState::Requesting)?)
    }

    async fn handle_state_requesting(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
        println!("Entering REQUESTING");
        // Discard other DHCPOFFER

        // We should get a DHCPACK or DHCPNAK message
        // TODO (Techassi): Scale the timeout duration over time
        let (message, _addr) =
            match utils::timeout(self.read_timeout, self.recv_message(&socket)).await {
                TimeoutResult::Timeout => {
                    self.transition_to(DhcpState::Init)?;
                    return Ok(());
                }
                TimeoutResult::Error(err) => return Err(err),
                TimeoutResult::Ok(result) => match result {
                    Some(result) => result,
                    None => return Ok(()),
                },
            };

        // TODO (Techassi): We should introduce a timer which ticks everytime we encounter this code path to
        // not get stuck in this state
        match message.get_message_type() {
            Some(ty) => match ty {
                DhcpMessageType::Nak => {
                    self.transition_to(DhcpState::Init)?;
                    return Ok(());
                }
                DhcpMessageType::Ack => {}
                _ => return Ok(()),
            },
            None => return Ok(()),
        }

        // Set lease, T1 and T2 timers (DHCPACK)
        match message.get_renewal_t1_time() {
            Some(time) => self.client_state.renewal_time = Some(*time),
            None => {
                // Fallback to 50% of offered IP address lease time for Renewal (T1) time
                let time = (self.client_state.offered_lease_time.unwrap() as f64 * 0.5) as u32;
                self.client_state.renewal_time = Some(time)
            }
        }

        match message.get_rebinding_t2_time() {
            Some(time) => self.client_state.rebinding_time = Some(*time),
            None => {
                // Fallback to 87.5% of offered IP address lease time for Rebinding (T2) time
                let time = (self.client_state.offered_lease_time.unwrap() as f64 * 0.875) as u32;
                self.client_state.rebinding_time = Some(time)
            }
        }

        println!(
            "ip -4 addr add {} dev {}",
            self.client_state.offered_ip_address.unwrap(),
            self.interface.name
        );
        cmd::add_ip_address(
            &self.client_state.offered_ip_address.unwrap(),
            &self.interface.name,
        )?;

        // Transition to BOUND
        Ok(self.transition_to(DhcpState::Bound)?)
    }

    /// Handle the DHCP state BOUND.
    async fn handle_state_bound(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
        println!("Entering BOUND");
        // Remain in this state. Discard incoming
        // DHCPOFFER, DHCPACK and DHCPNAK

        // T1 expires, send DHCPREQUEST to leasing server
        println!("Waiting for T1 to expire, then sending DHCPREQUEST");
        match &self.client_state.renewal_time {
            Some(time) => sleep(Duration::from_secs(*time as u64)).await,
            None => {
                return Err(ClientError::Invalid(String::from(
                    "BOUND: No renewal (T1) time set, invalid state",
                )))
            }
        }

        println!("Sending DHCPREQUEST");
        let request_message = self.make_request_message()?;
        self.send_message(request_message, socket).await?;

        // Transition to RENEWING
        Ok(self.transition_to(DhcpState::Renewing)?)
    }

    async fn handle_state_renewing(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
        println!("Entering RENEWING");
        // Set lease, T1 and T2 timers (DHCPACK)

        // DHCPNAK, return to INIT

        // T2 expires, broadcast DHCPREQUEST
        // Transition to REBINDING

        Ok(())
    }

    fn get_xid(&self) -> u32 {
        self.client_state.transaction_id
    }

    fn destination_addr(&self) -> Ipv4Addr {
        match self.client_state.server_identifier {
            Some(ip) => ip,
            None => Ipv4Addr::BROADCAST,
        }
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
        let mut message = Message::new_with_xid(self.get_xid());

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
        let mut message = Message::new_with_xid(self.get_xid());

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
            OptionData::ServerIdentifier(self.client_state.server_identifier.unwrap()),
        ))?;

        // Set DHCP requested IP address option
        message.add_option(DhcpOption::new(
            OptionTag::RequestedIpAddr,
            OptionData::RequestedIpAddr(self.client_state.offered_ip_address.unwrap()),
        ))?;

        // Set DHCP IP address lease time
        message.add_option(DhcpOption::new(
            OptionTag::IpAddrLeaseTime,
            OptionData::IpAddrLeaseTime(self.client_state.offered_lease_time.unwrap()),
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
