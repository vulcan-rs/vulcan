use std::time;

use network_interface::Error as InterfaceError;
use thiserror::Error;

use crate::{
    client::{cmd::CmdError, state::DhcpStateError},
    types::{MessageError, ParseHardwareAddrError},
};

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),

    #[error("Bind error: Failed to create and bind UDP socket after {0:?}")]
    BindTimeout(time::Duration),

    #[error("Failed to retrieve interfaces: {0}")]
    InterfaceError(#[from] InterfaceError),

    #[error("Failed to select a network interface: {0}")]
    NoInterfaceFound(String),

    #[error("Parse hardware address error: {0}")]
    ParseHardwareAddrError(#[from] ParseHardwareAddrError),

    #[error("No hardware/mac address on interface '{0}' found")]
    NoHardwareAddressError(String),

    #[error("DHCP state error: {0}")]
    DhcpStateError(#[from] DhcpStateError),

    #[error("Message error: {0}")]
    MessageError(#[from] MessageError),

    #[error("Command error: {0}")]
    CmdError(#[from] CmdError),

    #[error("Invalid message format or length: {0}")]
    Invalid(String),
}
