use std::time;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("IO error")]
    IO(#[from] std::io::Error),

    #[error("Bind error: Failed to create and bind UDP socket after {0:?}")]
    BindTimeout(time::Duration),

    #[error("Buffer error")]
    BufError(#[from] binbuf::BufferError),

    #[error("Invalid message format or length: {0}")]
    Invalid(String),
}
