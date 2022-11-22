use thiserror::Error;

#[derive(Debug, PartialEq, Error)]
pub enum BufferError {
    #[error("Max buffer length overflow")]
    MaxLengthOverflow,

    #[error("Invalid buffer jump index")]
    InvalidJumpIndex,

    #[error("Buffer too short")]
    BufTooShort,
}
