use std::{
    net::Ipv4Addr,
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
    client::state::{ClientState, DhcpState},
    constants,
    types::Message,
    DHCP_MINIMUM_LEGAL_MAX_MESSAGE_SIZE,
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
            bind_timeout: self.bind_timeout,
            read_timeout: self.read_timeout,
            write_timeout: self.write_timeout,
            state: ClientState::default(),
            interface: NetworkInterface::new_afinet("eth0", Ipv4Addr::UNSPECIFIED, None, None, 1),
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

    /// The client state, see
    /// https://www.rfc-editor.org/rfc/rfc2131#section-4.4
    state: ClientState,

    /// The selected network interface
    interface: NetworkInterface,
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
        let interface = interface.ok_or(ClientError::NoInterfaceFound)?;

        // Create UDP socket with a bind timeout
        let sock = create_sock_with_timeout("0.0.0.0:68", self.bind_timeout).await?;

        loop {
            // First try to retreive one (if any) UDP datagrams.
            // readable can produce a false positive, which is why we need to
            // check for errors when calling try_recv_from.
            sock.readable().await?;

            // Create an empty (all 0) buffer with the minimum legal max DHCP
            // message size
            let mut buf = vec![0u8; DHCP_MINIMUM_LEGAL_MAX_MESSAGE_SIZE.into()];

            let (buf, addr) = match sock.try_recv_from(&mut buf) {
                Ok((len, addr)) => (&buf[..len], addr),
                Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    continue;
                }
                Err(e) => {
                    return Err(e.into());
                }
            };

            let mut buf = ReadBuffer::new(buf);
            let response = Message::read_be(&mut buf)?;

            // We now use a state machine to keep track of the client state.
            // This is described in 4.4: https://www.rfc-editor.org/rfc/rfc2131#section-4.4
            match self.dhcp_state() {
                DhcpState::Init => {
                    // Wait a random amount between one and ten seconds
                    let mut rng = rand::thread_rng();
                    let wait_duration = Duration::from_secs(rng.gen_range(1..=10));
                    sleep(wait_duration).await;

                    let discover_msg = self.make_discover_message();

                    // Send DHCPDISCOVER message

                    // Transition to SELECTING
                    self.state.transition_to(DhcpState::Selecting)?;
                }
                DhcpState::InitReboot => todo!(),
                DhcpState::Selecting => {
                    // Collect replies (DHCPOFFER)

                    // Select offer

                    // Send DHCPREQUEST message

                    // Transition to REQUESTING
                }
                DhcpState::Rebooting => todo!(),
                DhcpState::Requesting => {
                    // Discard other DHCPOFFER

                    // Set lease, T1 and T2 timers (DHCPACK)

                    // Send DHCPACK message

                    // Transition to BOUND
                }
                DhcpState::Rebinding => {
                    // Set lease, T1 and T2 timers (DHCPACK)
                    // Transition to BOUND

                    // Lease expired (DHCPNAK), return to INIT
                }
                DhcpState::Bound => {
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

    /// Retrieve the DHCP state from the client state. This is a shortcut for
    /// `self.state.dhcp_state()`.
    fn dhcp_state(&self) -> &DhcpState {
        self.state.dhcp_state()
    }

    fn make_discover_message(&self) -> Message {
        Message::new()
    }

    /// Send a DHCP message / packet with the default timeouts to `dest_addr`
    /// by binding to `bind_addr`. The bind address is usually `0.0.0.0:68`.
    /// The default timeouts can be adjusted by using [`Client::builder`]
    async fn send<A>(&self, dest_addr: Ipv4Addr, bind_addr: A) -> Result<(), ClientError>
    where
        A: ToSocketAddrs,
    {
        // See Packet Processing - 7.1 Client Transmission
        // https://datatracker.ietf.org/doc/html/rfc951#section-7

        // Create UDP socket with a bind timeout
        let sock = create_sock_with_timeout(bind_addr, self.bind_timeout).await?;

        // If the provided destionation IP address is the broadcast address,
        // we set the socket to broadcast mode
        if dest_addr.is_broadcast() {
            if let Err(err) = sock.set_broadcast(true) {
                return Err(ClientError::IO(err));
            }
        }

        // Construct a new BOOTP message with default values
        let msg = Message::new();

        println!("{}", msg);

        // Create the write buffer
        let mut buf = WriteBuffer::new();

        // Write finished message to the buffer
        if let Err(err) = msg.write_be(&mut buf) {
            return Err(ClientError::MessageError(err));
        }

        // Assure the buffer is longer then the minimum DHCP message size
        if buf.len() < constants::MIN_DHCP_MSG_SIZE {
            return Err(ClientError::Invalid(
                "DHCP message is shorter than the minimum required length".into(),
            ));
        }

        // println!("{:02X?}", buf.bytes());

        // Off to the wire the bytes go
        let n = sock.send_to(buf.bytes(), (dest_addr, 67)).await?;
        println!("Bytes written: {}", n);

        // Start the receive loop. We try a certain amount of times until we
        // give up. After giving up, the caller needs to handle the DHCP
        // request failure. This usually involves to send a new DHCP request
        // after a few seconds / minutes.

        // loop {}

        Ok(())
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
