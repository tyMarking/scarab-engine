use thiserror::Error;
use uuid::Uuid;

use crate::gameobject::entity::registry::ControlError;

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
    #[error("Field positions must be positive")]
    FieldPosition,
    #[error(transparent)]
    ControlError(#[from] ControlError),
    #[error("Attempted to register an entity with a pre-existing UUID: {0}")]
    EntityRegistration(Uuid),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
