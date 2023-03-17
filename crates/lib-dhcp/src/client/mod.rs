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
    builder::MessageBuilder,
    client::state::{ClientState, DhcpState, DhcpStateMachine},
    types::{options::DhcpMessageType, HardwareAddr, Message, OptionData, OptionTag},
    utils, TimeoutResult, MINIMAL_RETRANS_DURATION_SECS, MINIMUM_LEGAL_MAX_MESSAGE_SIZE,
    SERVER_PORT,
};

mod cmd;
mod error;
mod state;
mod storage;
// mod timers;

pub use error::ClientError;

pub struct ClientBuilder {
    /// Duration before the binding process of the socket times out.
    bind_timeout: time::Duration,

    /// Duration before the read process of DHCP answers times out.
    read_timeout: time::Duration,

    /// Duration before the write process of DHCP requests times out.
    write_timeout: time::Duration,

    /// Optional client identifier, fallsback to the hardware addr.
    client_identifier: Option<Vec<u8>>,

    /// Max DHCP message size, default value is 1500.
    max_dhcp_message_size: u16,

    /// Fallback to appropriate alternative network interface if no interface
    /// with the provided name was found.
    interface_fallback: bool,

    /// Network interface name
    interface: String,
}

impl Default for ClientBuilder {
    fn default() -> Self {
        Self {
            bind_timeout: time::Duration::from_secs(2),
            read_timeout: time::Duration::from_secs(2),
            write_timeout: time::Duration::from_secs(2),
            interface: String::from("eth0"),
            max_dhcp_message_size: 1500,
            interface_fallback: false,
            client_identifier: None,
        }
    }
}

impl ClientBuilder {
    pub fn build(self) -> Result<Client, ClientError> {
        let interface =
            match utils::select_network_interface(&self.interface, self.interface_fallback)? {
                Some(ifa) => ifa,
                None => return Err(ClientError::NoInterfaceFound(self.interface)),
            };

        let hardware_address = match &interface.mac_addr {
            Some(mac_addr) => HardwareAddr::try_from(mac_addr)?,
            None => return Err(ClientError::NoHardwareAddressError(interface.name)),
        };

        let builder = MessageBuilder::new(
            hardware_address.clone(),
            self.client_identifier,
            self.max_dhcp_message_size,
        );

        Ok(Client {
            client_state: ClientState::default(),
            write_timeout: self.write_timeout,
            dhcp_state: DhcpState::default(),
            bind_timeout: self.bind_timeout,
            read_timeout: self.read_timeout,
            hardware_address,
            interface,
            builder,
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

    pub fn with_client_identifier<T: Into<Vec<u8>>>(mut self, identifier: T) -> Self {
        self.client_identifier = Some(identifier.into());
        self
    }

    pub fn with_max_dhcp_message_size(mut self, size: u16) -> Self {
        self.max_dhcp_message_size = size;
        self
    }
}

// TODO (Techassi): The T1 and T2 timers a implemented slightly wrong. See 4.4.5

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

    /// Message builder
    builder: MessageBuilder,
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

        // We use a state machine to keep track of the client state.
        // This is described in 4.4: https://www.rfc-editor.org/rfc/rfc2131#section-4.4

        // NOTE (Techassi): Maaaaan I really want to remove the multiple awaits and just
        //                  single one at the end of the match expression, but this
        //                  doesn't work for whatever reason...
        loop {
            match self.dhcp_state {
                DhcpState::Init => self.handle_init().await?,
                DhcpState::InitReboot => self.handle_init_reboot().await?, // NOOP
                DhcpState::Selecting => self.handle_selecting(&socket).await?,
                DhcpState::SelectingSent => self.handle_selecting_sent(&socket).await?,
                DhcpState::Rebooting => self.handle_rebooting().await?, // NOOP
                DhcpState::Requesting => self.handle_requesting(&socket).await?,
                DhcpState::RequestingSent => self.handle_requesting_sent(&socket).await?,
                DhcpState::Rebinding => self.handle_rebinding(&socket).await?,
                DhcpState::RebindingSent => self.handle_rebinding_sent(&socket).await?,
                DhcpState::Bound => self.handle_bound().await?,
                DhcpState::Renewing => self.handle_renewing(&socket).await?,
                DhcpState::RenewingSent => self.handle_renewing_sent(&socket).await?,
            }
        }
    }

    /// Handle the DHCP state INIT
    async fn handle_init(&mut self) -> Result<(), ClientError> {
        println!("Entering state INIT");
        // Wait a random amount between one and ten seconds
        let wait_duration = Duration::from_secs(rand::thread_rng().gen_range(1..=10));
        println!(
            "Waiting for {:?} to send DHCPDISCOVER message",
            wait_duration
        );
        sleep(wait_duration).await;

        // Transition to SELECTING
        Ok(self.transition_to(DhcpState::Selecting)?)
    }

    async fn handle_init_reboot(&mut self) -> Result<(), ClientError> {
        Ok(())
    }

    /// Handle the DHCP state SELECTING
    async fn handle_selecting(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
        println!("Entering state SELECTING");

        // Send DHCPDISCOVER message
        println!("Sending DHCPDISCOVER message");
        let discover_message = self.builder.make_discover_message(
            self.get_xid(),
            self.destination_addr(),
            None,
            None,
        )?;
        self.send_message(discover_message, &socket).await?;

        // Transition to REQUESTING
        Ok(self.transition_to(DhcpState::SelectingSent)?)
    }

    async fn handle_selecting_sent(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
        println!("Entering state SELECTING-SENT");
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
        if !message.valid_xid(self.get_xid()) {
            println!(
                "Received response with wrong transaction ID: {} (yours: {})",
                message.header.xid,
                self.get_xid()
            );
            return Ok(());
        }

        // Check if the DHCP message type is correct
        if !message.valid_message_type(DhcpMessageType::Offer) {
            println!("Received response with no DHCP message type option set");
            return Ok(());
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

        Ok(self.transition_to(DhcpState::Requesting)?)
    }

    async fn handle_rebooting(&mut self) -> Result<(), ClientError> {
        Ok(())
    }

    async fn handle_requesting(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
        println!("Entering REQUESTING");

        // Send DHCPREQUEST message
        println!("Sending DHCPREQUEST message");
        let request_message = self.builder.make_request_message(
            self.get_xid(),
            self.destination_addr(),
            self.client_state.offered_ip_address.unwrap(),
            self.client_state.offered_lease_time.unwrap(),
        )?;
        self.send_message(request_message, &socket).await?;

        Ok(self.transition_to(DhcpState::RequestingSent)?)
    }

    async fn handle_requesting_sent(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
        println!("Entering REQUESTING-SENT");
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

        // Check if the transaction ID matches
        if !message.valid_xid(self.get_xid()) {
            println!(
                "Received response with wrong transaction ID: {} (yours: {})",
                message.header.xid,
                self.get_xid()
            );
            return Ok(());
        }

        // TODO (Techassi): We should introduce a timer which ticks everytime we encounter this code path to
        // not get stuck in this state
        match message.get_message_type() {
            Some(ty) => match ty {
                DhcpMessageType::Nak => {
                    return Ok(self.transition_to(DhcpState::Init)?);
                }
                DhcpMessageType::Ack => {}
                _ => return Ok(()),
            },
            None => return Ok(()),
        }

        // Set lease, T1 and T2 timers (DHCPACK)
        self.client_state.renewal_time = Some(
            message
                .get_renewal_t1_time()
                .unwrap_or((self.client_state.offered_lease_time.unwrap() as f64 * 0.5) as u32),
        );

        self.client_state.rebinding_time = Some(
            message
                .get_rebinding_t2_time()
                .unwrap_or((self.client_state.offered_lease_time.unwrap() as f64 * 0.875) as u32),
        );

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

    async fn handle_rebinding(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
        println!("Entering REBINDING");

        // Reset the server identifier (IP address). The message will be
        // send using the broadcast address.
        self.client_state.server_identifier = None;

        println!("Sending DHCPREQUEST");
        let request_message = self.builder.make_renewing_message(
            self.get_xid(),
            self.client_state.offered_ip_address.unwrap(),
            self.client_state.offered_lease_time.unwrap(),
        )?;
        self.send_message(request_message, socket).await?;

        Ok(self.transition_to(DhcpState::RebindingSent)?)
    }

    async fn handle_rebinding_sent(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
        println!("Entering REBINDING-SENT");

        let (message, _addr) = match self.recv_message(socket).await? {
            Some(result) => result,
            None => match &self.client_state.rebinding_time_left {
                Some(time) => {
                    // We dropped below the minimal retransmission timer,
                    // transition to INIT.
                    if *time < MINIMAL_RETRANS_DURATION_SECS * 2 {
                        return Ok(self.transition_to(DhcpState::Init)?);
                    }

                    // We still have time left to receive a response.
                    sleep(Duration::from_secs(*time as u64)).await;
                    self.client_state.rebinding_time_left = Some((time / 2) as u32);

                    return Ok(self.transition_to(DhcpState::Rebinding)?);
                }
                None => {
                    return Err(ClientError::Invalid(String::from(
                        "RENEWING: No renewal (T1) timer",
                    )))
                }
            },
        };

        // Check if the transaction ID matches
        if !message.valid_xid(self.get_xid()) {
            println!(
                "Received response with wrong transaction ID: {} (yours: {})",
                message.header.xid,
                self.get_xid()
            );
            return Ok(());
        }

        match message.get_message_type() {
            Some(ty) => match ty {
                DhcpMessageType::Nak => {
                    self.transition_to(DhcpState::Init)?;
                    return Ok(());
                }
                DhcpMessageType::Ack => {}
                _ => return Ok(()), // NOTE (Techassi): How should we handle other message types?
            },
            None => return Ok(()),
        }

        // Set lease, T1 and T2 timers (DHCPACK)
        self.client_state.renewal_time = Some(
            message
                .get_renewal_t1_time()
                .unwrap_or((self.client_state.offered_lease_time.unwrap() as f64 * 0.5) as u32),
        );

        self.client_state.rebinding_time = Some(
            message
                .get_rebinding_t2_time()
                .unwrap_or((self.client_state.offered_lease_time.unwrap() as f64 * 0.875) as u32),
        );

        println!(
            "ip -4 addr add {} dev {}",
            self.client_state.offered_ip_address.unwrap(),
            self.interface.name
        );
        cmd::add_ip_address(
            &self.client_state.offered_ip_address.unwrap(),
            &self.interface.name,
        )?;

        Ok(self.transition_to(DhcpState::Bound)?)
    }

    /// Handle the DHCP state BOUND.
    async fn handle_bound(&mut self) -> Result<(), ClientError> {
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

        // Transition to RENEWING
        Ok(self.transition_to(DhcpState::Renewing)?)
    }

    /// Handle the DHCP state RENEWING. This method sends out the DHCP message
    /// and immediatly transitions to the intermediate state RENEWING-SENT.
    /// This state is officially not part of the state machine described by
    /// RFC 2131, but this implementation introduces this state to be able to
    /// return back to here in case the T1 timer ticks which should trigger a
    /// retransmission of the DHCPREQUEST message.
    async fn handle_renewing(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
        println!("Entering RENEWING");
        println!("Renewing XID");
        self.renew_xid();

        println!("Sending DHCPREQUEST");
        let request_message = self.builder.make_renewing_message(
            self.get_xid(),
            self.client_state.offered_ip_address.unwrap(),
            self.client_state.offered_lease_time.unwrap(),
        )?;
        self.send_message(request_message, socket).await?;

        Ok(self.transition_to(DhcpState::RenewingSent)?)
    }

    /// Handle the intermediate state RENEWINGSENT. This method listens for
    /// incoming messages after sending out a DHCPREQUEST message to renew the
    /// lease. If
    async fn handle_renewing_sent(&mut self, socket: &UdpSocket) -> Result<(), ClientError> {
        println!("Entering RENEWING-SENT");

        let (message, _addr) = match self.recv_message(socket).await? {
            Some(result) => result,
            None => match &self.client_state.renewal_time_left {
                Some(time) => {
                    // We dropped below the minimal retransmission timer,
                    // transition to REBINDING.
                    if *time < MINIMAL_RETRANS_DURATION_SECS * 2 {
                        return Ok(self.transition_to(DhcpState::Rebinding)?);
                    }

                    // We still have time left to receive a response.
                    sleep(Duration::from_secs(*time as u64)).await;
                    self.client_state.renewal_time_left = Some((time / 2) as u32);

                    return Ok(self.transition_to(DhcpState::Renewing)?);
                }
                None => {
                    return Err(ClientError::Invalid(String::from(
                        "RENEWING: No renewal (T1) timer",
                    )))
                }
            },
        };

        // Check if the transaction ID matches
        if !message.valid_xid(self.get_xid()) {
            println!(
                "Received response with wrong transaction ID: {} (yours: {})",
                message.header.xid,
                self.get_xid()
            );
            return Ok(());
        }

        // TODO (Techassi): All this stuff below can be extracted into a method
        // Set lease, T1 and T2 timers (DHCPACK)
        match message.get_message_type() {
            Some(ty) => match ty {
                DhcpMessageType::Nak => {
                    self.transition_to(DhcpState::Init)?;
                    return Ok(());
                }
                DhcpMessageType::Ack => {}
                _ => return Ok(()), // NOTE (Techassi): How should we handle other message types?
            },
            None => return Ok(()),
        }

        // Set lease, T1 and T2 timers (DHCPACK)
        self.client_state.renewal_time = Some(
            message
                .get_renewal_t1_time()
                .unwrap_or((self.client_state.offered_lease_time.unwrap() as f64 * 0.5) as u32),
        );

        self.client_state.rebinding_time = Some(
            message
                .get_rebinding_t2_time()
                .unwrap_or((self.client_state.offered_lease_time.unwrap() as f64 * 0.875) as u32),
        );

        println!(
            "ip -4 addr add {} dev {}",
            self.client_state.offered_ip_address.unwrap(),
            self.interface.name
        );
        cmd::add_ip_address(
            &self.client_state.offered_ip_address.unwrap(),
            &self.interface.name,
        )?;

        Ok(self.transition_to(DhcpState::Bound)?)
    }

    /// Returns the current transaction ID.
    fn get_xid(&self) -> u32 {
        self.client_state.transaction_id
    }

    /// Renews the transaction ID by selecting a new, random one.
    fn renew_xid(&mut self) {
        self.client_state.transaction_id = rand::random()
    }

    /// Returns the destination address. This is either the IP address of the
    /// current DHCP server or the IPv4 broadcast address.
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
        let mut buf = vec![0u8; MINIMUM_LEGAL_MAX_MESSAGE_SIZE.into()];

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
            .send_to(buf.bytes(), (destination_addr, SERVER_PORT))
            .await?;

        Ok(())
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
