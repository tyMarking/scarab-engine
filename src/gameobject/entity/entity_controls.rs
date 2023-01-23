use crate::Velocity;

#[derive(Debug, Clone)]
pub enum EntityControls {
    SetMovement(Velocity),
    Nop,
}
