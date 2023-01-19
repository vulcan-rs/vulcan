use std::time;

use thiserror::Error;

use crate::types::MessageError;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("IO error {0}")]
    IO(#[from] std::io::Error),

    #[error("Bind error: Failed to create and bind UDP socket after {0:?}")]
    BindTimeout(time::Duration),

    #[error("Buffer error")]
    MessageError(#[from] MessageError),

    #[error("Invalid message format or length: {0}")]
    Invalid(String),
}
