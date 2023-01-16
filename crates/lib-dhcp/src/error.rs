use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProtocolError {
    #[error("Invalid opcode ({0})")]
    InvalidOpCode(u8),
}
