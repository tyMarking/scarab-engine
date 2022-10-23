use core::ops::{Add, Mul, Sub};

/// float based RGBA [0, 1]
pub type Color = [f32; 4];

pub trait VecNum = Sized
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + PartialOrd
    + PartialEq
    + Clone
    + Copy
    + From<u32>
    + Into<f64>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct TileVec<N: VecNum> {
    pub x: N,
    pub y: N,
}

impl<N: VecNum> TileVec<N> {
    pub fn new(x: N, y: N) -> Self {
        Self { x, y }
    }

    pub fn x(&self) -> N {
        self.x
    }

    pub fn y(&self) -> N {
        self.y
    }

    // Directly implementing `From<Vec2<N>> for Vec2<M>` produces a conflicting
    // implementation in the case where N = M, so add this function instead
    pub fn convert_n<M>(self) -> TileVec<M>
    where
        M: VecNum,
        N: Into<M>,
    {
        TileVec {
            x: self.x.into(),
            y: self.y.into(),
        }
    }
}

impl<N: VecNum> From<TileVec<N>> for (N, N) {
    fn from(val: TileVec<N>) -> (N, N) {
        (val.x(), val.y())
    }
}

impl<N: VecNum> Add<TileVec<N>> for TileVec<N> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let x = self.x() + rhs.x();
        let y = self.y() + rhs.y();
        Self::new(x, y)
    }
}

impl<N: VecNum> Sub<TileVec<N>> for TileVec<N> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        let x = self.x() - rhs.x();
        let y = self.y() - rhs.y();
        Self::new(x, y)
    }
}

impl<N: VecNum> Mul<N> for TileVec<N> {
    type Output = Self;
    fn mul(self, rhs: N) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PhysBox<N: VecNum> {
    pos: TileVec<N>,
    size: TileVec<N>,
}

impl<N: VecNum> PhysBox<N> {
    pub fn new(x: N, y: N, w: N, h: N) -> Self {
        Self {
            pos: TileVec::new(x, y),
            size: TileVec::new(w, h),
        }
    }

    pub fn from_pos(pos: TileVec<N>, w: N, h: N) -> Self {
        Self {
            pos,
            size: TileVec::new(w, h),
        }
    }

    pub fn from_pos_size(pos: TileVec<N>, size: TileVec<N>) -> Self {
        Self { pos, size }
    }

    pub fn pos(&self) -> TileVec<N> {
        self.pos
    }

    pub fn set_pos(&mut self, pos: TileVec<N>) {
        self.pos = pos;
    }

    pub fn size(&self) -> TileVec<N> {
        self.size
    }

    pub fn set_size(&mut self, size: TileVec<N>) {
        self.size = size;
    }

    pub fn left_x(&self) -> N {
        self.pos().x()
    }

    pub fn right_x(&self) -> N {
        self.pos().x() + self.size().x()
    }

    pub fn top_y(&self) -> N {
        self.pos().y()
    }

    pub fn bottom_y(&self) -> N {
        self.pos().y() + self.size().y()
    }

    /// For clarity this uses < and > not <= and >=
    pub fn contains_pos(&self, pos: TileVec<N>) -> bool {
        let bottom_right = self.pos + self.size;
        (pos.x() > self.pos().x())
            && (pos.x() < bottom_right.x())
            && (pos.y() > self.pos().y())
            && (pos.y() < bottom_right.y())
    }

    /// For clarity this uses < and > not <= and >=
    pub fn has_overlap(&self, other: &Self) -> bool {
        let this_bottom_right = self.pos + self.size;
        let other_bottom_right = other.pos + other.size;
        (other_bottom_right.x() > self.pos().x())
            && (other.pos().x() < this_bottom_right.x())
            && (other_bottom_right.y() > self.pos().y())
            && (other.pos().y() < this_bottom_right.y())
    }

    // Is `self` fully contained within `other`
    pub fn is_fully_contained(&self, other: &Self) -> bool {
        other.contains_pos(self.pos) && other.contains_pos(self.pos + self.size)
    }

    // Directly implementing `From<Vec2<N>> for Vec2<M>` produces a conflicting
    // implementation in the case where N = M, so add this function instead
    pub fn convert_n<M>(self) -> PhysBox<M>
    where
        M: VecNum,
        N: Into<M>,
    {
        PhysBox {
            pos: self.pos.convert_n(),
            size: self.size.convert_n(),
        }
    }
}

pub trait HasBox<N: VecNum> {
    fn get_box(&self) -> &PhysBox<N>;
}

pub trait HasBoxMut<N: VecNum> {
    fn get_box_mut(&mut self) -> &mut PhysBox<N>;
}

#[cfg(test)]
mod test {}
