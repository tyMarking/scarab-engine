use graphics::types::{Scalar, Vec2d};
use serde::{Deserialize, Serialize};
use shapes::{Point, Size};

use super::BoxEdge;
use crate::{Axis, PhysicsError, PhysicsResult};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PhysBox {
    pos: Point,
    size: Size,
}

impl PhysBox {
    fn validate(_point: Point, size: Size) -> PhysicsResult<()> {
        if size.w > 0.0 && size.h > 0.0 {
            Ok(())
        } else {
            Err(PhysicsError::PhysBoxSize)
        }
    }

    pub fn new([x, y, w, h]: [f64; 4]) -> PhysicsResult<Self> {
        let pos = [x, y].into();
        let size = [w, h].into();
        Self::validate(pos, size)?;
        Ok(Self { pos, size })
    }

    pub fn pos(&self) -> Point {
        self.pos
    }

    pub fn pos_mut(&mut self) -> &mut Point {
        &mut self.pos
    }

    pub fn set_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn set_size(&mut self, size: Size) -> PhysicsResult<()> {
        Self::validate(self.pos, size)?;
        self.size = size;
        Ok(())
    }

    /// Gets the y coordinate of the top edge
    pub fn top_y(&self) -> Scalar {
        self.pos.y
    }

    /// Gets the x coordinate of the left edge
    pub fn left_x(&self) -> Scalar {
        self.pos.x
    }

    /// Gets the y coordinate of the bottom edge
    pub fn bottom_y(&self) -> Scalar {
        self.pos.y + self.size.h
    }

    /// Gets the x coordinate of the right edge
    pub fn right_x(&self) -> Scalar {
        self.pos.x + self.size.w
    }

    /// Translates self so that the y value of the top edge is 'val'
    pub fn set_top_y(&mut self, val: Scalar) {
        self.pos.y = val;
    }

    /// Translates self so that the x value of the left edge is 'val'
    pub fn set_left_x(&mut self, val: Scalar) {
        self.pos.x = val;
    }

    /// Translates self so that the y value of the bottom edge is 'val'
    pub fn set_bottom_y(&mut self, val: Scalar) {
        self.pos.y = val - self.size.w;
    }

    /// Translates self so that the x value of the right edge is 'val'
    pub fn set_right_x(&mut self, val: Scalar) {
        self.pos.x = val - self.size.w;
    }

    /// Computes the area of self
    pub fn area(&self) -> Scalar {
        self.size.w * self.size.h
    }

    /// Gets the corresponding coordinate for the given edge of the box.
    /// i.e. top/bottom give their respective y's; left/right give their respective x's
    pub fn get_edge(&self, edge: BoxEdge) -> Scalar {
        match edge {
            BoxEdge::Top => self.top_y(),
            BoxEdge::Left => self.left_x(),
            BoxEdge::Bottom => self.bottom_y(),
            BoxEdge::Right => self.right_x(),
        }
    }

    /// Moves `self` so that `self.get_edge(edge) == val`
    pub fn set_edge(&mut self, val: Scalar, edge: BoxEdge) {
        match edge {
            BoxEdge::Top => self.set_top_y(val),
            BoxEdge::Left => self.set_left_x(val),
            BoxEdge::Bottom => self.set_bottom_y(val),
            BoxEdge::Right => self.set_right_x(val),
        }
    }

    /// Moves `self` so that its and `other`'s `edge` coincide
    pub fn set_touching_edge(&mut self, other: &Self, edge: BoxEdge) {
        self.set_edge(other.get_edge(edge), edge)
    }

    /// Moves `self` so that its `this_edge` coincides with `other`'s `this_edge.opposite()`
    pub fn set_touching_opposite_edge(&mut self, other: &Self, this_edge: BoxEdge) {
        self.set_edge(other.get_edge(this_edge.opposite()), this_edge)
    }

    /// Gets the corresponding component of the bottom right corner
    pub fn get_far_axis(&self, axis: Axis) -> Scalar {
        match axis {
            Axis::X => self.right_x(),
            Axis::Y => self.bottom_y(),
        }
    }

    /// Gets the corresponding component of the top left corner
    pub fn get_near_axis(&self, axis: Axis) -> Scalar {
        match axis {
            Axis::X => self.left_x(),
            Axis::Y => self.top_y(),
        }
    }

    /// Moves `self` so it does not overlap with `other`.
    /// Does nothing if they already don't overlap.
    /// ```
    /// use scarab_engine::PhysBox;
    ///
    /// let box1 = PhysBox::new([0.0, 0.0, 5.0, 5.0].into()).unwrap();
    /// let mut box2 = PhysBox::new([2.0, 4.0, 5.0, 5.0].into()).unwrap();
    ///
    /// assert!(box1.has_overlap(&box2));
    ///
    /// box2.shift_to_nonoverlapping(&box1);
    /// // Even though box1 and box2 still touch on box1's right axis they don't overlap
    /// assert_eq!(box2, PhysBox::new([2.0, 5.0, 5.0, 5.0].into()).unwrap());
    /// ```
    pub fn shift_to_nonoverlapping(&mut self, other: &Self) {
        if !self.has_overlap(other) {
            return;
        }
        let diffs = vec![
            (BoxEdge::Top, other.bottom_y() - self.top_y()),
            (BoxEdge::Left, other.right_x() - self.left_x()),
            (BoxEdge::Bottom, self.bottom_y() - other.top_y()),
            (BoxEdge::Right, self.right_x() - other.left_x()),
        ];

        let shift_edge_opt = diffs.iter().fold(None, |smallest_edge, (edge, diff)| {
            if Into::<f64>::into(*diff) > 0.0
                && smallest_edge.map_or_else(|| true, |(_, s)| diff < s)
            {
                Some((edge, diff))
            } else {
                smallest_edge
            }
        });

        if let Some((edge, _diff)) = shift_edge_opt {
            self.set_touching_opposite_edge(other, *edge)
        }
    }

    /// Is the pos contained in the box
    /// For clarity this uses >= and <
    /// i.e. The top and left edges are inclusive, and the bottom and right ones are exclusive
    /// with the bottom left and top right corners also excluded.
    pub fn contains_pos(&self, pos: Point) -> bool {
        let bottom_right: Point = (self.pos + Point::from(Vec2d::from(self.size))).into();
        (pos.x >= self.pos.x)
            && (pos.x < bottom_right.x)
            && (pos.y >= self.pos.y)
            && (pos.y < bottom_right.y)
    }

    /// Is the pos contained in the box, (with all edges included)
    fn contains_pos_inclusive(&self, pos: Point) -> bool {
        let bottom_right: Point = self.pos + self.size;
        (pos.x >= self.pos.x)
            && (pos.x <= bottom_right.x)
            && (pos.y >= self.pos.y)
            && (pos.y <= bottom_right.y)
    }

    /// Do self and other have any overlap.
    /// For clarity this uses < and > not <= and >=, i.e. two cells that share
    /// an edge do not overlap.
    /// ```
    /// use scarab_engine::PhysBox;
    ///
    /// let box1 = PhysBox::new([0.0, 0.0, 5.0, 5.0].into()).unwrap();
    /// let mut box2 = PhysBox::new([3.0, 3.0, 5.0, 5.0].into()).unwrap();
    ///
    /// assert!(box1.has_overlap(&box2));
    ///
    /// box2.shift_to_nonoverlapping(&box1);
    /// // Even though box1 and box2 still touch on box1's right axis they don't overlap
    /// assert!(!box1.has_overlap(&box2));
    /// ```
    pub fn has_overlap(&self, other: &Self) -> bool {
        let this_bottom_right = self.pos + self.size;
        let other_bottom_right = other.pos + other.size;
        (other_bottom_right.x > self.pos.x)
            && (other.pos.x < this_bottom_right.x)
            && (other_bottom_right.y > self.pos.y)
            && (other.pos.y < this_bottom_right.y)
    }

    /// Is `self` fully contained within `other`
    /// Uses fully inclusive logic so that a.is_fully_contained_by(&a) is true
    /// i.e. in set notation `a.is_fully_contained_by(&b)` means that $a \subset b$
    pub fn is_fully_contained_by(&self, other: &Self) -> bool {
        other.contains_pos_inclusive(self.pos) && other.contains_pos_inclusive(self.pos + self.size)
    }

    /// Returns a list of the edges of `self` that `other` touches.
    /// (todo: test) Will be empty iff `other` is fully contained by `self` or they have no overlap
    pub fn edges_crossed_by(&self, other: &Self) -> Vec<BoxEdge> {
        if !self.has_overlap(other) || other.is_fully_contained_by(self) {
            return vec![];
        }
        let mut edges = Vec::with_capacity(4);

        if self.is_top_edge_crossed_by(other) {
            edges.push(BoxEdge::Top);
        }
        if self.is_left_edge_crossed_by(other) {
            edges.push(BoxEdge::Left);
        }
        if self.is_bottom_edge_crossed_by(other) {
            edges.push(BoxEdge::Bottom);
        }
        if self.is_right_edge_crossed_by(other) {
            edges.push(BoxEdge::Right);
        }
        edges
    }

    pub fn is_edge_crossed_by(&self, other: &Self, edge: BoxEdge) -> bool {
        match edge {
            BoxEdge::Top => self.is_top_edge_crossed_by(other),
            BoxEdge::Left => self.is_left_edge_crossed_by(other),
            BoxEdge::Bottom => self.is_bottom_edge_crossed_by(other),
            BoxEdge::Right => self.is_right_edge_crossed_by(other),
        }
    }

    pub fn is_top_edge_crossed_by(&self, other: &Self) -> bool {
        other.pos.y < self.pos.y && other.bottom_y() > self.pos.y
    }

    pub fn is_left_edge_crossed_by(&self, other: &Self) -> bool {
        other.pos.x < self.pos.x && other.right_x() > self.pos.x
    }

    pub fn is_bottom_edge_crossed_by(&self, other: &Self) -> bool {
        other.pos.y < self.bottom_y() && other.bottom_y() > self.bottom_y()
    }

    pub fn is_right_edge_crossed_by(&self, other: &Self) -> bool {
        other.pos.x < self.right_x() && other.right_x() > self.right_x()
    }
}

impl HasBox for PhysBox {
    fn get_box(&self) -> &PhysBox {
        self
    }
}

impl HasBoxMut for PhysBox {
    fn get_box_mut(&mut self) -> &mut PhysBox {
        self
    }
}

/// A trait for game objects that wrap a physbox
pub trait HasBox {
    fn get_box(&self) -> &PhysBox;
}

/// A trait for game objects that wrap a mutable physbox
pub trait HasBoxMut {
    fn get_box_mut(&mut self) -> &mut PhysBox;
}

#[cfg(test)]
mod test {
    use crate::PhysicsError;

    use super::*;

    #[test]
    fn negative_sized_rect_not_allowed() {
        assert_eq!(
            PhysBox::new([0.0, 0.0, 1.0, -4.0]).unwrap_err(),
            PhysicsError::PhysBoxSize
        );
        assert_eq!(
            PhysBox::new([0.0, 0.0, -0.1, 4.0]).unwrap_err(),
            PhysicsError::PhysBoxSize
        );
        assert_eq!(
            PhysBox::new([0.0, 0.0, -10000.0, -2.0]).unwrap_err(),
            PhysicsError::PhysBoxSize
        );
    }

    #[test]
    fn zero_sized_rect_not_allowed() {
        assert_eq!(
            PhysBox::new([0.0, 0.0, 0.0, 0.0]).unwrap_err(),
            PhysicsError::PhysBoxSize
        );
        assert_eq!(
            PhysBox::new([0.0, 0.0, 0.0, 4.0]).unwrap_err(),
            PhysicsError::PhysBoxSize
        );
        assert_eq!(
            PhysBox::new([0.0, 0.0, 1.0, 0.0]).unwrap_err(),
            PhysicsError::PhysBoxSize
        );
    }

    #[test]
    /// All Points on the top and left edges excluding those on right or bottom corner
    /// are contained in the box
    fn contains_pos_contains_top_left_edges() {
        let physbox1 = PhysBox::new([0.0, 0.0, 3.0, 4.0]).unwrap();
        assert!(physbox1.contains_pos([0.0, 0.0].into()));
        assert!(physbox1.contains_pos([0.0, 1.0].into()));
        assert!(physbox1.contains_pos([0.0, 2.0].into()));
        assert!(physbox1.contains_pos([0.0, 3.0].into()));
        assert!(physbox1.contains_pos([1.0, 0.0].into()));
        assert!(physbox1.contains_pos([2.0, 0.0].into()));

        let physbox2 = PhysBox::new([1.0, 1.0, 4.0, 5.0]).unwrap();
        assert!(physbox2.contains_pos([1.0, 1.0].into()));
        assert!(physbox2.contains_pos([1.0, 2.0].into()));
        assert!(physbox2.contains_pos([1.0, 3.0].into()));
        assert!(physbox2.contains_pos([1.0, 4.0].into()));
        assert!(physbox2.contains_pos([1.0, 4.99].into()));
        assert!(physbox2.contains_pos([2.0, 1.0].into()));
        assert!(physbox2.contains_pos([3.0, 1.0].into()));
        assert!(physbox2.contains_pos([3.99, 1.0].into()));
    }

    #[test]
    /// Points that are not on an edge are contained in the box
    fn contains_pos_contains_middle() {
        let physbox1 = PhysBox::new([0.0, 0.0, 5.0, 5.0]).unwrap();
        assert!(physbox1.contains_pos([2.0, 2.0].into()));

        let physbox2 = PhysBox::new([1.0, 1.0, 5.0, 5.0]).unwrap();
        assert!(physbox2.contains_pos([3.0, 3.0].into()));
        assert!(physbox2.contains_pos([5.99, 5.99].into()));
    }
    #[test]
    /// All points on the bottom and right edges are not contained
    fn contains_pos_doesnt_contain_bottom_right_edges() {
        let physbox1 = PhysBox::new([0.0, 0.0, 3.0, 4.0]).unwrap();
        assert!(!physbox1.contains_pos([0.0, 4.0].into()));
        assert!(!physbox1.contains_pos([1.0, 4.0].into()));
        assert!(!physbox1.contains_pos([2.0, 4.0].into()));
        assert!(!physbox1.contains_pos([3.0, 4.0].into()));
        assert!(!physbox1.contains_pos([3.0, 0.0].into()));
        assert!(!physbox1.contains_pos([3.0, 1.0].into()));
        assert!(!physbox1.contains_pos([3.0, 2.0].into()));
        assert!(!physbox1.contains_pos([3.0, 3.0].into()));

        let physbox2 = PhysBox::new([1.0, 1.0, 3.0, 4.0]).unwrap();
        assert!(!physbox2.contains_pos([1.0, 5.0].into()));
        assert!(!physbox2.contains_pos([2.0, 5.0].into()));
        assert!(!physbox2.contains_pos([3.0, 5.0].into()));
        assert!(!physbox2.contains_pos([4.0, 5.0].into()));
        assert!(!physbox2.contains_pos([4.0, 1.0].into()));
        assert!(!physbox2.contains_pos([4.0, 2.0].into()));
        assert!(!physbox2.contains_pos([4.0, 3.0].into()));
        assert!(!physbox2.contains_pos([4.0, 4.0].into()));
    }

    #[test]
    /// All points obviously not in the box are not contained
    fn contains_pos_doesnt_contain_obvious() {
        let physbox1 = PhysBox::new([0.0, 0.0, 3.0, 4.0]).unwrap();
        assert!(!physbox1.contains_pos([0.0, 10.0].into()));
        assert!(!physbox1.contains_pos([10.0, 0.0].into()));
        assert!(!physbox1.contains_pos([10.0, 10.0].into()));
        assert!(!physbox1.contains_pos([3.0, 10.0].into()));
        assert!(!physbox1.contains_pos([10.0, 4.0].into()));

        let physbox2 = PhysBox::new([1.0, 1.0, 2.0, 2.0]).unwrap();
        assert!(!physbox2.contains_pos([0.0, 0.0].into()));
        assert!(!physbox2.contains_pos([-5.0, -5.0].into()));
        assert!(!physbox2.contains_pos([1.0, 10.0].into()));
        assert!(!physbox2.contains_pos([10.0, 3.0].into()));
    }

    #[test]
    fn has_overlap_works() {
        let physbox1 = PhysBox::new([1.0, 0.0, 3.0, 3.0]).unwrap();
        let physbox2 = PhysBox::new([0.0, 1.0, 5.0, 1.0]).unwrap();
        let physbox3 = PhysBox::new([2.0, 1.0, 1.0, 1.0]).unwrap();
        let physbox4 = PhysBox::new([2.0, 1.0, 2.0, 2.0]).unwrap();

        assert!(physbox1.has_overlap(&physbox1));
        assert!(physbox1.has_overlap(&physbox2));
        assert!(physbox1.has_overlap(&physbox3));
        assert!(physbox1.has_overlap(&physbox4));
    }

    #[test]
    fn has_overlap_adjacent_cells_dont_overlap() {
        let physbox0_0 = PhysBox::new([0.0, 0.0, 5.0, 5.0]).unwrap();
        let physbox0_1 = PhysBox::new([0.0, 5.0, 5.0, 5.0]).unwrap();
        let physbox1_0 = PhysBox::new([5.0, 0.0, 5.0, 5.0]).unwrap();
        let physbox1_1 = PhysBox::new([5.0, 5.0, 5.0, 5.0]).unwrap();

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
        let physbox = PhysBox::new([1.0, 50.0, 20.0, 20.0]).unwrap();

        assert!(physbox.is_fully_contained_by(&physbox));
    }

    #[test]
    fn box_containment_not_commutative() {
        let physbox1 = PhysBox::new([0.0, 0.0, 20.0, 20.0]).unwrap();
        let physbox2 = PhysBox::new([5.0, 5.0, 5.0, 5.0]).unwrap();

        assert!(physbox2.is_fully_contained_by(&physbox1));
        assert!(!physbox1.is_fully_contained_by(&physbox2));
    }

    #[test]
    fn box_on_top_left_edge_is_contained() {
        let physbox1 = PhysBox::new([0.0, 0.0, 20.0, 20.0]).unwrap();
        let physbox2 = PhysBox::new([0.0, 0.0, 10.0, 10.0]).unwrap();
        let physbox3 = PhysBox::new([5.0, 0.0, 10.0, 10.0]).unwrap();
        let physbox4 = PhysBox::new([0.0, 5.0, 10.0, 10.0]).unwrap();

        assert!(physbox2.is_fully_contained_by(&physbox1));
        assert!(physbox3.is_fully_contained_by(&physbox1));
        assert!(physbox4.is_fully_contained_by(&physbox1));
    }

    #[test]
    fn box_on_bottom_right_edge_is_contained() {
        let physbox1 = PhysBox::new([0.0, 0.0, 20.0, 20.0]).unwrap();
        let physbox2 = PhysBox::new([10.0, 10.0, 10.0, 10.0]).unwrap();
        let physbox3 = PhysBox::new([10.0, 0.0, 10.0, 10.0]).unwrap();
        let physbox4 = PhysBox::new([0.0, 10.0, 10.0, 10.0]).unwrap();

        assert!(physbox2.is_fully_contained_by(&physbox1));
        assert!(physbox3.is_fully_contained_by(&physbox1));
        assert!(physbox4.is_fully_contained_by(&physbox1));
    }
}
