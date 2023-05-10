/// All moving/kinetic objects in a game scene
pub mod entity;

/// The setting of a game scene, determines static obstables
pub mod field;

use core::ops::{BitAnd, BitOr, Not};

pub use entity::Entity;
pub use field::{Cell, Field};
use graphics::types::Scalar;
use serde::{Deserialize, Serialize};

use crate::BoxEdge;

/// Represents whether the typical entity can enter/exit a cell from each side
///
/// The structure of a solidity is as follows:
/// The most signicant 4 bits represent the "exitability" for the game object
/// while the least significant 4 bits represent the "enterability".
/// Within each octet the bits from most to least significant are: left, right, top, bottom.
/// When a bit is 1 that means the edge can be entered/exited
///
/// However, it is likely easier add together the given constants in some way.
/// ```
/// use scarab_engine::gameobject::{SOLID, ENTER_LEFT, ENTER_RIGHT, ENTER_TOP, ENTER_BOTTOM};
/// // A solidity that can do anything except enter on the left
/// let cant_enter_left = !ENTER_LEFT;
/// assert!(!cant_enter_left.enter_left());
/// assert!(cant_enter_left.exit_left());
///
/// // A solidity that can be entered from any edge, but can't exit any edge
/// let enter_all_cant_exit = ENTER_LEFT | ENTER_RIGHT | ENTER_TOP | ENTER_BOTTOM;
/// assert!(enter_all_cant_exit.enter_left());
/// assert!(enter_all_cant_exit.enter_right());
/// assert!(enter_all_cant_exit.enter_top());
/// assert!(enter_all_cant_exit.enter_bottom());
/// assert!(!enter_all_cant_exit.exit_left());
/// assert!(!enter_all_cant_exit.exit_right());
/// assert!(!enter_all_cant_exit.exit_top());
/// assert!(!enter_all_cant_exit.exit_bottom());
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Solidity(pub u8);

/// Solidity for game objects that can't be entered or exited from any side
pub const SOLID: Solidity = Solidity(0);
/// Solidity for game objects that can be entered or exited from any side
pub const NO_SOLIDITY: Solidity = Solidity(255);
/// Bitmask for solidities that can enter on the left
pub const ENTER_LEFT: Solidity = Solidity(0b0000_1000);
/// Bitmask for solidities that can enter on the left
pub const EXIT_LEFT: Solidity = Solidity(0b1000_0000);
/// Bitmask for solidities that can exit on the right
pub const ENTER_RIGHT: Solidity = Solidity(0b0000_0100);
/// Bitmask for solidities that can enter on the right
pub const EXIT_RIGHT: Solidity = Solidity(0b0100_0000);
/// Bitmask for solidities that can exit on the top
pub const ENTER_TOP: Solidity = Solidity(0b0000_0010);
/// Bitmask for solidities that can exit on the top
pub const EXIT_TOP: Solidity = Solidity(0b0010_0000);
/// Bitmask for solidities that can enter on the bottom
pub const ENTER_BOTTOM: Solidity = Solidity(0b0000_0001);
/// Bitmask for solidities that can exit on the bottom
pub const EXIT_BOTTOM: Solidity = Solidity(0b0001_0000);

/// For each function true means that edge can be passed
impl Solidity {
    /// Whether or not the left side of the attached object can be entered.
    #[inline]
    pub fn enter_left(&self) -> bool {
        self.0 & ENTER_LEFT.0 != 0
    }

    /// Whether or not the left side of the attached object can be exited.
    #[inline]
    pub fn exit_left(&self) -> bool {
        self.0 & EXIT_LEFT.0 != 0
    }

    /// Whether or not the right side of the attached object can be entered.
    #[inline]
    pub fn enter_right(&self) -> bool {
        self.0 & ENTER_RIGHT.0 != 0
    }

    /// Whether or not the right side of the attached object can be exited.
    #[inline]
    pub fn exit_right(&self) -> bool {
        self.0 & EXIT_RIGHT.0 != 0
    }

    /// Whether or not the top side of the attached object can be entered.
    #[inline]
    pub fn enter_top(&self) -> bool {
        self.0 & ENTER_TOP.0 != 0
    }

    /// Whether or not the top side of the attached object can be exited.
    #[inline]
    pub fn exit_top(&self) -> bool {
        self.0 & EXIT_TOP.0 != 0
    }

    /// Whether or not the bottom side of the attached object can be entered.
    #[inline]
    pub fn enter_bottom(&self) -> bool {
        self.0 & ENTER_BOTTOM.0 != 0
    }

    /// Whether or not the bottom side of the attached object can be exited.
    #[inline]
    pub fn exit_bottom(&self) -> bool {
        self.0 & EXIT_BOTTOM.0 != 0
    }

    /// Whether the given edge of the attached object can be entered.
    pub fn enter_edge(&self, edge: BoxEdge) -> bool {
        match edge {
            BoxEdge::Top => self.enter_top(),
            BoxEdge::Left => self.enter_left(),
            BoxEdge::Bottom => self.enter_bottom(),
            BoxEdge::Right => self.enter_right(),
        }
    }

    /// Whether the given edge of the attached object can be exited.
    pub fn exit_edge(&self, edge: BoxEdge) -> bool {
        match edge {
            BoxEdge::Top => self.exit_top(),
            BoxEdge::Left => self.exit_left(),
            BoxEdge::Bottom => self.exit_bottom(),
            BoxEdge::Right => self.exit_right(),
        }
    }

    /// Returns true if there is any edge that can't be entered or exited
    pub fn has_solidity(&self) -> bool {
        self != &NO_SOLIDITY
    }
}

impl BitAnd<Solidity> for Solidity {
    type Output = Solidity;
    fn bitand(self, rhs: Solidity) -> Self::Output {
        Solidity(self.0 & rhs.0)
    }
}

impl BitOr<Solidity> for Solidity {
    type Output = Solidity;
    fn bitor(self, rhs: Solidity) -> Self::Output {
        Solidity(self.0 | rhs.0)
    }
}

impl Not for Solidity {
    type Output = Solidity;
    fn not(self) -> Self::Output {
        Solidity(!self.0)
    }
}

/// A trait for gameobjects that have a solidity component
pub trait HasSolidity {
    /// The game object's solidity component
    fn get_solidity(&self) -> &Solidity;
}

#[derive(Debug, Serialize, Deserialize)]
/// The health of a game object
pub struct Health {
    curr: Scalar,
    max: Scalar,
}

impl Health {
    /// Creates a new health, with the current value initialized at max.
    pub fn new(max: Scalar) -> Self {
        Self {
            curr: max,
            max: max,
        }
    }

    /// Apply a raw amount of damage.
    pub fn raw_damage(&mut self, amt: Scalar) {
        self.curr -= amt;
    }

    /// The current health value
    pub fn current(&self) -> Scalar {
        self.curr
    }

    /// The maximum health
    pub fn max(&self) -> Scalar {
        self.max
    }

    /// The current health as a fraction of max health
    pub fn fraction(&self) -> Scalar {
        self.curr / self.max
    }
}

/// A trait for gameobjects that have a health component
pub trait HasHealth {
    /// A reference to the game object's internal health
    fn get_health(&self) -> &Health;

    /// A mutable reference to the game object's internal health
    fn get_health_mut(&mut self) -> &mut Health;
}
