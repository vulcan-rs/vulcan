#[derive(Debug, PartialEq)]
pub enum BufferError {
    MaxLengthOverflow,
    InvalidJumpIndex,
    BufTooShort,
}
