use shapes::Point;
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
    #[error(transparent)]
    ControlError(#[from] ControlError),
    #[error("Attempted to register an entity with a pre-existing UUID: {0}")]
    EntityRegistration(Uuid),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    PhysicsError(#[from] PhysicsError),
}

pub type PhysicsResult<T> = Result<T, PhysicsError>;

#[derive(Debug, Error, PartialEq)]
pub enum PhysicsError {
    #[error("PhysBox sizes must be greater than 0")]
    PhysBoxSize,
    #[error("Field positions must be positive")]
    FieldPosition,
    #[error("Maximum velocity must be positive")]
    MaxVelocity,
    #[error("Could not find field cell at position {0:?}")]
    NoFieldCell(Point),
    #[error("Error indexing into 'field' with index {0}")]
    FieldIndex(usize),
}
