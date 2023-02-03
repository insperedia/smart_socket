use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TransportError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),
}
