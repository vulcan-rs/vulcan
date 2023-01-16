use std::{net::Ipv4Addr, time};

use tokio::{
    net::{ToSocketAddrs, UdpSocket},
    time::timeout,
};

use binbuf::prelude::*;

use crate::{constants, types::Message};

mod error;

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

    /// Send a DHCP message / packet with the default timeouts to `dest_addr`
    /// by binding to `bind_addr`. The bind address is usually `0.0.0.0:68`.
    /// The default timeouts can be adjusted by using [`Client::builder`]
    pub async fn send<A>(&self, dest_addr: Ipv4Addr, bind_addr: A) -> Result<(), ClientError>
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
            return Err(ClientError::BufError(err));
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
