use core::ops::{Add, Mul, Sub};

use crate::{ScarabError, ScarabResult};

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
    fn validate((_x, _y): (N, N), (w, h): (N, N)) -> ScarabResult<()> {
        if w.into() > 0.0 && h.into() > 0.0 {
            Ok(())
        } else {
            Err(ScarabError::PhysBoxSize)
        }
    }

    pub fn new((x, y): (N, N), (w, h): (N, N)) -> ScarabResult<Self> {
        Self::validate((x, y), (w, h))?;
        Ok(Self {
            pos: TileVec::new(x, y),
            size: TileVec::new(w, h),
        })
    }

    pub fn pos(&self) -> TileVec<N> {
        self.pos
    }

    pub fn set_pos(&mut self, pos: TileVec<N>) -> ScarabResult<()> {
        Self::validate(pos.into(), self.size.into())?;
        self.pos = pos;
        Ok(())
    }

    pub fn size(&self) -> TileVec<N> {
        self.size
    }

    pub fn set_size(&mut self, size: TileVec<N>) -> ScarabResult<()> {
        Self::validate(self.pos.into(), size.into())?;
        self.size = size;
        Ok(())
    }

    pub fn left_x(&self) -> N {
        self.pos.x()
    }

    pub fn right_x(&self) -> N {
        self.pos.x() + self.size.x()
    }

    pub fn top_y(&self) -> N {
        self.pos.y()
    }

    pub fn bottom_y(&self) -> N {
        self.pos.y() + self.size.y()
    }

    pub fn area(&self) -> N {
        self.size.x() * self.size.y()
    }

    /// For clarity this uses >= and <
    /// i.e. The top and left edges are inclusive, and the bottom and right ones are exclusive
    /// with the bottom left and top right corners also excluded.
    pub fn contains_pos(&self, pos: TileVec<N>) -> bool {
        let bottom_right = self.pos + self.size;
        (pos.x() >= self.pos().x())
            && (pos.x() < bottom_right.x())
            && (pos.y() >= self.pos().y())
            && (pos.y() < bottom_right.y())
    }

    /// Is the pos contained in the box, (with all edges included)
    fn contains_pos_inclusive(&self, pos: TileVec<N>) -> bool {
        let bottom_right = self.pos + self.size;
        (pos.x() >= self.pos().x())
            && (pos.x() <= bottom_right.x())
            && (pos.y() >= self.pos().y())
            && (pos.y() <= bottom_right.y())
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

    /// Is `self` fully contained within `other`
    /// Uses fully inclusive logic so that a.is_fully_contained_by(&a) is true
    /// i.e. in set notation `a.is_fully_contained_by(&b)` means that $a \subset b$
    pub fn is_fully_contained_by(&self, other: &Self) -> bool {
        other.contains_pos_inclusive(self.pos) && other.contains_pos_inclusive(self.pos + self.size)
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
mod test {
    use super::*;

    #[test]
    /// All Points on the top and left edges excluding those on right or bottom corner
    /// are contained in the box
    fn contains_pos_contains_top_left_edges() {
        let physbox1: PhysBox<u32> = PhysBox::new((0, 0), (3, 4)).unwrap();
        assert!(physbox1.contains_pos(TileVec::new(0, 0)));
        assert!(physbox1.contains_pos(TileVec::new(0, 1)));
        assert!(physbox1.contains_pos(TileVec::new(0, 2)));
        assert!(physbox1.contains_pos(TileVec::new(0, 3)));
        assert!(physbox1.contains_pos(TileVec::new(1, 0)));
        assert!(physbox1.contains_pos(TileVec::new(2, 0)));

        let physbox2 = PhysBox::new((1.0, 1.0), (4.0, 5.0)).unwrap();
        assert!(physbox2.contains_pos(TileVec::new(1.0, 1.0)));
        assert!(physbox2.contains_pos(TileVec::new(1.0, 2.0)));
        assert!(physbox2.contains_pos(TileVec::new(1.0, 3.0)));
        assert!(physbox2.contains_pos(TileVec::new(1.0, 4.0)));
        assert!(physbox2.contains_pos(TileVec::new(1.0, 4.99)));
        assert!(physbox2.contains_pos(TileVec::new(2.0, 1.0)));
        assert!(physbox2.contains_pos(TileVec::new(3.0, 1.0)));
        assert!(physbox2.contains_pos(TileVec::new(3.99, 1.0)));
    }

    #[test]
    /// Points that are not on an edge are contained in the box
    fn contains_pos_contains_middle() {
        let physbox1: PhysBox<u32> = PhysBox::new((0, 0), (5, 5)).unwrap();
        assert!(physbox1.contains_pos(TileVec::new(2, 2)));

        let physbox2 = PhysBox::new((1.0, 1.0), (5.0, 5.0)).unwrap();
        assert!(physbox2.contains_pos(TileVec::new(3.0, 3.0)));
        assert!(physbox2.contains_pos(TileVec::new(5.99, 5.99)));
    }
    #[test]
    /// All points on the bottom and right edges are not contained
    fn contains_pos_doesnt_contain_bottom_right_edges() {
        let physbox1: PhysBox<u32> = PhysBox::new((0, 0), (3, 4)).unwrap();
        assert!(!physbox1.contains_pos(TileVec::new(0, 4)));
        assert!(!physbox1.contains_pos(TileVec::new(1, 4)));
        assert!(!physbox1.contains_pos(TileVec::new(2, 4)));
        assert!(!physbox1.contains_pos(TileVec::new(3, 4)));
        assert!(!physbox1.contains_pos(TileVec::new(3, 0)));
        assert!(!physbox1.contains_pos(TileVec::new(3, 1)));
        assert!(!physbox1.contains_pos(TileVec::new(3, 2)));
        assert!(!physbox1.contains_pos(TileVec::new(3, 3)));

        let physbox2 = PhysBox::new((1.0, 1.0), (3.0, 4.0)).unwrap();
        assert!(!physbox2.contains_pos(TileVec::new(1.0, 5.0)));
        assert!(!physbox2.contains_pos(TileVec::new(2.0, 5.0)));
        assert!(!physbox2.contains_pos(TileVec::new(3.0, 5.0)));
        assert!(!physbox2.contains_pos(TileVec::new(4.0, 5.0)));
        assert!(!physbox2.contains_pos(TileVec::new(4.0, 1.0)));
        assert!(!physbox2.contains_pos(TileVec::new(4.0, 2.0)));
        assert!(!physbox2.contains_pos(TileVec::new(4.0, 3.0)));
        assert!(!physbox2.contains_pos(TileVec::new(4.0, 4.0)));
    }

    #[test]
    /// All points obviously not in the box are not contained
    fn contains_pos_doesnt_contain_obvious() {
        let physbox1: PhysBox<u32> = PhysBox::new((0, 0), (3, 4)).unwrap();
        assert!(!physbox1.contains_pos(TileVec::new(0, 10)));
        assert!(!physbox1.contains_pos(TileVec::new(10, 0)));
        assert!(!physbox1.contains_pos(TileVec::new(10, 10)));
        assert!(!physbox1.contains_pos(TileVec::new(3, 10)));
        assert!(!physbox1.contains_pos(TileVec::new(10, 4)));

        let physbox2 = PhysBox::new((1.0, 1.0), (2.0, 2.0)).unwrap();
        assert!(!physbox2.contains_pos(TileVec::new(0.0, 0.0)));
        assert!(!physbox2.contains_pos(TileVec::new(-5.0, -5.0)));
        assert!(!physbox2.contains_pos(TileVec::new(1.0, 10.0)));
        assert!(!physbox2.contains_pos(TileVec::new(10.0, 3.0)));
    }

    #[test]
    fn has_overlap_works() {
        let physbox1: PhysBox<u32> = PhysBox::new((1, 0), (3, 3)).unwrap();
        let physbox2: PhysBox<u32> = PhysBox::new((0, 1), (5, 1)).unwrap();
        let physbox3: PhysBox<u32> = PhysBox::new((2, 1), (1, 1)).unwrap();
        let physbox4: PhysBox<u32> = PhysBox::new((2, 1), (2, 2)).unwrap();

        assert!(physbox1.has_overlap(&physbox1));
        assert!(physbox1.has_overlap(&physbox2));
        assert!(physbox1.has_overlap(&physbox3));
        assert!(physbox1.has_overlap(&physbox4));
    }

    #[test]
    fn has_overlap_adjacent_cells_dont_overlap() {
        let physbox0_0: PhysBox<u32> = PhysBox::new((0, 0), (5, 5)).unwrap();
        let physbox0_1: PhysBox<u32> = PhysBox::new((0, 5), (5, 5)).unwrap();
        let physbox1_0: PhysBox<u32> = PhysBox::new((5, 0), (5, 5)).unwrap();
        let physbox1_1: PhysBox<u32> = PhysBox::new((5, 5), (5, 5)).unwrap();

        assert!(!physbox0_0.has_overlap(&physbox0_1));
        assert!(!physbox0_0.has_overlap(&physbox1_0));
        assert!(!physbox0_0.has_overlap(&physbox1_1));

        assert!(!physbox0_1.has_overlap(&physbox0_0));
        assert!(!physbox0_1.has_overlap(&physbox1_0));
        assert!(!physbox0_1.has_overlap(&physbox1_1));

        assert!(!physbox1_0.has_overlap(&physbox0_0));
        assert!(!physbox1_0.has_overlap(&physbox0_1));
        assert!(!physbox1_0.has_overlap(&physbox1_1));

        assert!(!physbox1_1.has_overlap(&physbox0_0));
        assert!(!physbox1_1.has_overlap(&physbox0_1));
        assert!(!physbox1_1.has_overlap(&physbox1_0));
    }

    #[test]
    fn box_contains_itself() {
        let physbox = PhysBox::new((1.0, 50.0), (20.0, 20.0)).unwrap();

        assert!(physbox.is_fully_contained_by(&physbox));
    }

    #[test]
    fn box_containment_not_commutative() {
        let physbox1 = PhysBox::new((0.0, 0.0), (20.0, 20.0)).unwrap();
        let physbox2 = PhysBox::new((5.0, 5.0), (5.0, 5.0)).unwrap();

        assert!(physbox2.is_fully_contained_by(&physbox1));
        assert!(!physbox1.is_fully_contained_by(&physbox2));
    }

    #[test]
    fn box_on_top_left_edge_is_contained() {
        let physbox1 = PhysBox::new((0.0, 0.0), (20.0, 20.0)).unwrap();
        let physbox2 = PhysBox::new((0.0, 0.0), (10.0, 10.0)).unwrap();
        let physbox3 = PhysBox::new((5.0, 0.0), (10.0, 10.0)).unwrap();
        let physbox4 = PhysBox::new((0.0, 5.0), (10.0, 10.0)).unwrap();

        assert!(physbox2.is_fully_contained_by(&physbox1));
        assert!(physbox3.is_fully_contained_by(&physbox1));
        assert!(physbox4.is_fully_contained_by(&physbox1));
    }

    #[test]
    fn box_on_bottom_right_edge_is_contained() {
        let physbox1 = PhysBox::new((0.0, 0.0), (20.0, 20.0)).unwrap();
        let physbox2 = PhysBox::new((10.0, 10.0), (10.0, 10.0)).unwrap();
        let physbox3 = PhysBox::new((10.0, 0.0), (10.0, 10.0)).unwrap();
        let physbox4 = PhysBox::new((0.0, 10.0), (10.0, 10.0)).unwrap();

        assert!(physbox2.is_fully_contained_by(&physbox1));
        assert!(physbox3.is_fully_contained_by(&physbox1));
        assert!(physbox4.is_fully_contained_by(&physbox1));
    }
}
