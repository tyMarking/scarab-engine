use core::ops::{Add, Mul, Sub};

use graphics::types::{Scalar, Vec2d};
use serde::{Deserialize, Serialize};
use shapes::Point;

pub mod physbox;

pub use physbox::*;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BoxEdge {
    Top,
    Left,
    Bottom,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(usize)]
pub enum Axis {
    X,
    Y,
}

impl BoxEdge {
    pub fn opposite(&self) -> BoxEdge {
        match self {
            Self::Top => Self::Bottom,
            Self::Left => Self::Right,
            Self::Bottom => Self::Top,
            Self::Right => Self::Left,
        }
    }

    pub fn normal_vector(&self) -> [Scalar; 2] {
        match self {
            BoxEdge::Top => [0.0, -1.0],
            BoxEdge::Left => [-1.0, 0.0],
            BoxEdge::Bottom => [0.0, 1.0],
            BoxEdge::Right => [1.0, 0.0],
        }
    }

    /// The axis that runs parallel to this edge's normal vector
    pub fn parallel_axis(&self) -> Axis {
        match self {
            BoxEdge::Top | BoxEdge::Bottom => Axis::Y,
            BoxEdge::Left | BoxEdge::Right => Axis::X,
        }
    }

    /// The axis that runs perpendicular to this edge's normal vector
    pub fn perpendicular_axis(&self) -> Axis {
        match self {
            BoxEdge::Top | BoxEdge::Bottom => Axis::X,
            BoxEdge::Left | BoxEdge::Right => Axis::Y,
        }
    }

    /// An iterator over each of the four edges
    pub fn iter() -> core::slice::Iter<'static, BoxEdge> {
        static EDGES: [BoxEdge; 4] = [BoxEdge::Top, BoxEdge::Left, BoxEdge::Bottom, BoxEdge::Right];
        EDGES.iter()
    }

    pub fn get_normal_component_of(&self, point: &Point) -> Scalar {
        match self {
            BoxEdge::Top | BoxEdge::Bottom => point.x,
            BoxEdge::Left | BoxEdge::Right => point.y,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Velocity {
    pub x: Scalar,
    pub y: Scalar,
}

impl Velocity {
    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Self { x: 0.0, y: 0.0 }
        } else {
            Self {
                x: self.x / mag,
                y: self.y / mag,
            }
        }
    }

    pub fn magnitude_sq(&self) -> Scalar {
        self.x * self.x + self.y * self.y
    }

    pub fn magnitude(&self) -> Scalar {
        f64::sqrt(self.x * self.x + self.y * self.y)
    }

    /// Whether this velocity would be reduced by colliding with a game object on the given edge
    /// i.e. is the dot product of the velocity and the edge's normal negative
    pub fn is_reduced_by_edge(&self, edge: BoxEdge) -> bool {
        match edge {
            BoxEdge::Top => self.y < 0.0,
            BoxEdge::Left => self.x < 0.0,
            BoxEdge::Bottom => self.y > 0.0,
            BoxEdge::Right => self.x > 0.0,
        }
    }

    /// Gives the angle of the velocity vector in radians
    /// with positive radians being from +x to +y. This would be clockwise on
    /// the display because +y is down.
    pub fn angle(&self) -> Scalar {
        f64::atan2(self.y, self.x)
    }
}

impl<T: Into<Velocity>> Add<T> for Velocity {
    type Output = Velocity;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Into<Velocity>> Sub<T> for Velocity {
    type Output = Velocity;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = rhs.into();
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add<Velocity> for Point {
    type Output = Point;

    fn add(self, vel: Velocity) -> Self::Output {
        Self {
            x: self.x + vel.x,
            y: self.y + vel.y,
        }
    }
}

impl Mul<Scalar> for Velocity {
    type Output = Velocity;

    fn mul(self, rhs: Scalar) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl From<Vec2d> for Velocity {
    fn from(v: Vec2d) -> Self {
        Self { x: v[0], y: v[1] }
    }
}

pub trait HasUuid {
    fn uuid(&self) -> Uuid;
}
