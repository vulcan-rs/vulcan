use rustyline::error::ReadlineError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReplError {
    #[error("Readline error: {0}")]
    ReadlineError(#[from] ReadlineError),
}
