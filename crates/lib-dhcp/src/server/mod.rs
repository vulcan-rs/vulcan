use std::{net::SocketAddr, sync::Arc};

use binbuf::prelude::*;
use thiserror::Error;
use tokio::{self, net};

use crate::{
    constants,
    types::{options::DhcpMessageType, Message},
};

mod builder;
mod storage;

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

            let mut buf = [0u8; constants::MINIMUM_LEGAL_MAX_MESSAGE_SIZE as usize];
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

    let message_type = match message.get_message_type() {
        Some(ty) => ty,
        None => {
            println!("No DHCP message type option");
            return;
        }
    };

    match message_type {
        DhcpMessageType::Discover => handle_discover(message, session).await,
        DhcpMessageType::Offer => handle_offer(message, session).await,
        DhcpMessageType::Request => handle_request(message, session).await,
        DhcpMessageType::Decline => handle_decline(message, session).await,
        DhcpMessageType::Ack => handle_ack(message, session).await,
        DhcpMessageType::Nak => handle_nak(message, session).await,
        DhcpMessageType::Release => handle_release(message, session).await,
    }
}

async fn handle_discover(message: Message, session: Session) {
    todo!()
}

async fn handle_offer(message: Message, session: Session) {
    todo!()
}

async fn handle_request(message: Message, session: Session) {
    todo!()
}

async fn handle_decline(message: Message, session: Session) {
    todo!()
}

async fn handle_ack(message: Message, session: Session) {
    todo!()
}

async fn handle_nak(message: Message, session: Session) {
    todo!()
}

async fn handle_release(message: Message, session: Session) {
    todo!()
}
