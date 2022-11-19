pub mod entity;
pub mod field;

pub use entity::Entity;
pub use field::{Cell, Field};

use crate::BoxEdge;

/// Represents whether the typical entity can enter/exit a cell from each side
/// The typical solid cell will be 255 or 15, while the typical navigable cell
/// will be 0.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Solidity(pub u8);

pub const SOLID: Solidity = Solidity(255);
pub const AIR: Solidity = Solidity(0);

/// TODO: create bitmasks for the solidity.

/// For each function true means that edge can be passed
impl Solidity {
    #[inline]
    pub fn enter_left(&self) -> bool {
        self.0 & 0b0000_1000 == 0
    }

    #[inline]
    pub fn exit_left(&self) -> bool {
        self.0 & 0b1000_0000 == 0
    }

    #[inline]
    pub fn enter_right(&self) -> bool {
        self.0 & 0b0000_0100 == 0
    }

    #[inline]
    pub fn exit_right(&self) -> bool {
        self.0 & 0b0100_0000 == 0
    }

    #[inline]
    pub fn enter_top(&self) -> bool {
        self.0 & 0b0000_0010 == 0
    }

    #[inline]
    pub fn exit_top(&self) -> bool {
        self.0 & 0b0010_0000 == 0
    }

    #[inline]
    pub fn enter_bottom(&self) -> bool {
        self.0 & 0b0000_0001 == 0
    }

    #[inline]
    pub fn exit_bottom(&self) -> bool {
        self.0 & 0b0001_0000 == 0
    }

    pub fn enter_edge(&self, edge: BoxEdge) -> bool {
        match edge {
            BoxEdge::Top => self.enter_top(),
            BoxEdge::Left => self.enter_left(),
            BoxEdge::Bottom => self.enter_bottom(),
            BoxEdge::Right => self.enter_right(),
        }
    }

    pub fn exit_edge(&self, edge: BoxEdge) -> bool {
        match edge {
            BoxEdge::Top => self.exit_top(),
            BoxEdge::Left => self.exit_left(),
            BoxEdge::Bottom => self.exit_bottom(),
            BoxEdge::Right => self.exit_right(),
        }
    }

    // Returns true if there is any edge that can't be entered or exited
    pub fn has_solidity(&self) -> bool {
        self.0 != 0
    }
}

pub trait HasSolidity {
    fn get_solidity(&self) -> &Solidity;
}

#[derive(Debug)]
pub struct Health {
    curr: u32,
    max: u32,
}

impl Health {
    pub fn new(max: u32) -> Self {
        Self {
            curr: max,
            max: max,
        }
    }

    /// Apply a raw amount of damage. Returns Ok(()) if the new current is > 0,
    /// and Err(remaining: u32) otherwise
    pub fn raw_damage(&mut self, amt: u32) -> Result<(), u32> {
        if self.curr > amt {
            self.curr -= amt;
            Ok(())
        } else {
            Err(amt - self.curr)
        }
    }
}

pub trait HasHealth {
    fn get_health(&self) -> &Health;
}
