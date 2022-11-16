use std::{net::Ipv4Addr, sync::Arc};

use crate::{constants, types::message::Message};
use binbuf::{ToWriteBuffer, WriteBuffer, Writeable};
use tokio::net::{ToSocketAddrs, UdpSocket};

mod error;

pub use error::ClientError;

pub struct Client {
    /// Internal read / write buffer.
    buf: Vec<u8>,

    /// Internal UDP socket.
    sock: Arc<UdpSocket>,
}

impl Client {
    /// Create a new BOOTP client with default values.
    pub async fn new<A: ToSocketAddrs>(sock_addr: A) -> Result<Self, ClientError> {
        let sock = match UdpSocket::bind(sock_addr).await {
            Ok(sock) => sock,
            Err(err) => {
                return Err(ClientError::new(format!(
                    "Failed to create and bind client UDP socket: {}",
                    err
                )))
            }
        };

        // Wrapp the socket into an Arc to be able to use it across multiple
        // transmissions
        let sock = Arc::new(sock);

        let client = Self {
            buf: vec![0; constants::BOOTP_MSG_SIZE],
            sock,
        };

        Ok(client)
    }

    /// Clear the internal read / write buffer.
    pub fn clear_buffer(&mut self) {
        self.buf.clear();
    }

    /// Send a BOOTP message / packet to `dest_addr`.
    pub fn send(&self, dest_addr: Ipv4Addr) -> Result<(), ClientError> {
        // See Packet Processing - 7.1 Client Transmission
        // https://datatracker.ietf.org/doc/html/rfc951#section-7

        // First we clear the internal message / packet buffer as suggested
        // by the RFC
        let mut buf = WriteBuffer::new();

        // If the provided destionation IP address is the broadcast address,
        // we set the socket to broadcast mode
        if dest_addr.is_broadcast() {
            if let Err(err) = self.sock.set_broadcast(true) {
                return Err(ClientError::new(format!(
                    "Failed to enable broadcast mode of the client socket: {}",
                    err
                )));
            }
        }

        // Construct a new BOOTP message with default values
        let msg = Message::new(0, 0, 0);

        println!("{}", msg);

        // Write finished message to the buffer
        if let Err(_) = msg.write(&mut buf) {
            return Err(ClientError::new("Failed to write message to buffer"));
        }

        // Assure the buffer is longer then the maximum BOOTP message size
        if buf.len() > constants::BOOTP_MSG_SIZE {
            return Err(ClientError::new(
                "Message to longer than maximum BOOTP message size",
            ));
        }

        println!("{:02X?}", buf.bytes());

        Ok(())
    }
}
