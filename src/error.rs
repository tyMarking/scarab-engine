use std::sync::mpsc::TryRecvError;

use thiserror::Error;

pub type ScarabResult<T> = Result<T, ScarabError>;

#[derive(Debug, Error, PartialEq)]
pub enum ScarabError {
    #[error("Unable to get a GPU Adapter")]
    RequestAdapterError,
    #[error("Unknown application error")]
    Unknown,
    #[error("{0}")]
    RawString(String),
    #[error("PhysBox sizes must be greater than 0")]
    PhysBoxSize,
    #[error("Field positions must be positive")]
    FieldPosition,
    #[error(transparent)]
    ChannelRecv(#[from] TryRecvError),
}
