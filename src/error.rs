use shapes::Point;
use thiserror::Error;
use uuid::Uuid;

/// A genric result for Scarab Engine operations
pub type ScarabResult<T> = Result<T, ScarabError>;

#[derive(Debug, Error)]
/// A generic error for Scarab Engine operations
pub enum ScarabError {
    #[error("Unable to get a GPU Adapter")]
    /// GPU loading failed
    RequestAdapterError,
    #[error("Unknown application error")]
    /// Some unknown error
    Unknown,
    #[error("{0}")]
    /// A raw string, should only be used for development purposes. When possible, make your own error variant
    RawString(String),
    #[error("Attempted to register an entity with a pre-existing UUID: {0}")]
    /// Registering a new entity failed
    EntityRegistration(Uuid),
    #[error(transparent)]
    /// I/O Errors
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    /// Errors related to the physics engine
    PhysicsError(#[from] PhysicsError),
}

/// A generic result type for physics operations
pub type PhysicsResult<T> = Result<T, PhysicsError>;

#[derive(Debug, Error, PartialEq)]
/// A generic error type for physics operations
pub enum PhysicsError {
    #[error("PhysBox sizes must be greater than 0")]
    /// Occurs when a physbox size results in undefined behavior
    PhysBoxSize,
    #[error("Field positions must be positive")]
    /// Occurs when a position on a field results in undefined behavior
    FieldPosition,
    #[error("Maximum velocity must be positive")]
    /// Occurs when an invalid maximum velocity is set
    MaxVelocity,
    #[error("Could not find field cell at position {0:?}")]
    /// Occurs when there is no cell on the field at the given point
    NoFieldCell(Point),
    #[error("Error indexing into 'field' with index {0}")]
    /// Occurs when there is no cell on the field with the given index
    FieldIndex(usize),
}
