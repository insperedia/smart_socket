use std::error::Error;
use thiserror::Error;

//pub type SendResult = Result<(), SendError>;

use std::io;
use std::io::ErrorKind;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}
