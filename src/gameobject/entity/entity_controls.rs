#[derive(Debug, Clone)]
pub enum EntityControls {
    SetMovement([f64; 2]),
    Nop,
}
