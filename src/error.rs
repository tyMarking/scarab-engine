use thiserror::Error;

pub type ScarabResult<T> = Result<T, ScarabError>;

#[derive(Debug, Error)]
pub enum ScarabError {
    #[error("Unable to get a GPU Adapter")]
    RequestAdapterError,
    #[error("Unknown application error")]
    Unknown,
    #[error("{0}")]
    RawString(String),
    #[error("PhysBox sizes must be greater than 0")]
    PhysBoxSize,
}
