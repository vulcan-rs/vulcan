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

use crate::{client::state::ClientState, constants, types::Message};

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
        if interface.is_none() {
            return Err(ClientError::NoInterfaceFound);
        }

        self.interface = interface.unwrap();

        // Create UDP socket with a bind timeout
        let sock = create_sock_with_timeout("0.0.0.0:68", self.bind_timeout).await?;

        loop {
            // We now use a state machine to keep track of the client state.
            // This is described in 4.4: https://www.rfc-editor.org/rfc/rfc2131#section-4.4
            match self.state {
                ClientState::Init => {
                    // Wait a random amount between one and ten seconds
                    let mut rng = rand::thread_rng();
                    let wait_duration = Duration::from_secs(rng.gen_range(1..=10));
                    sleep(wait_duration).await;

                    let discover_msg = self.make_discover_message();
                    break;
                }
                ClientState::InitReboot => todo!(),
                ClientState::Selecting => todo!(),
                ClientState::Rebooting => todo!(),
                ClientState::Requesting => todo!(),
                ClientState::Rebinding => todo!(),
                ClientState::Bound => todo!(),
                ClientState::Renewing => todo!(),
            }
        }

        Ok(())
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
