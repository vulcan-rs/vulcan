use std::{net::SocketAddr, sync::Arc};

use binbuf::prelude::*;
use thiserror::Error;
use tokio::{self, net};

use crate::{constants, types::Message};

pub struct Session {
    socket: Arc<net::UdpSocket>,
    addr: SocketAddr,
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("Server is already running")]
    AlreadyRunning,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct Server {
    is_running: bool,
}

impl Server {
    pub fn new() -> Self {
        Self { is_running: false }
    }

    #[tokio::main]
    pub async fn run(&mut self) -> Result<(), ServerError> {
        if self.is_running {
            return Err(ServerError::AlreadyRunning);
        }
        self.is_running = true;

        let socket = match net::UdpSocket::bind("0.0.0.0:67").await {
            Ok(socket) => socket,
            Err(err) => return Err(ServerError::Io(err)),
        };

        let socket = Arc::new(socket);

        loop {
            // Wait until the socket is readable, this can produce a false positive
            socket.readable().await?;

            let mut buf = [0u8; constants::MIN_DHCP_MSG_SIZE];
            let (len, addr) = match socket.recv_from(&mut buf).await {
                Ok(result) => result,
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock => {
                    // Continue when the socket.readable() call procduced a
                    // false positive
                    continue;
                }
                Err(err) => {
                    // TODO (Techassi): Log this
                    println!("{}", err);
                    continue;
                }
            };

            let session = Session {
                socket: socket.clone(),
                addr,
            };

            tokio::spawn(async move {
                handle(&buf[..len], session).await;
            });
        }
    }
}

async fn handle(buf: &[u8], session: Session) {
    let mut buf = ReadBuffer::new(buf);

    let message = match Message::read::<BigEndian>(&mut buf) {
        Ok(msg) => msg,
        Err(err) => {
            println!("Error while reading DHCP message: {}", err);
            return;
        }
    };

    println!("{}", message)
}
