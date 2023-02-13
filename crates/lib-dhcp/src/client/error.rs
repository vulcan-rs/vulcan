use std::time;

use network_interface::Error as InterfaceError;
use thiserror::Error;

use crate::types::MessageError;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("IO error {0}")]
    IO(#[from] std::io::Error),

    #[error("Bind error: Failed to create and bind UDP socket after {0:?}")]
    BindTimeout(time::Duration),

    #[error("Failed to retrieve interfaces: {0}")]
    InterfaceError(#[from] InterfaceError),

    #[error("Failed to select a network interface")]
    NoInterfaceFound,

    #[error("Message error: {0}")]
    MessageError(#[from] MessageError),

    #[error("Invalid message format or length: {0}")]
    Invalid(String),
}
