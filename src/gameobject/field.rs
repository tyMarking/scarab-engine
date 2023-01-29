use graphics::{types::Color, Context};
/// A field is a graph of rectangles (cells) that each have a (backup) rendering
/// component and a collision (or non-collision) component
///
/// The cells have a solidity field (defined in mod.rs) which dictates
/// how standard entities can move between cells.
/// In the construction of a gamefield from the given cells, their solidity is
/// used to construct a graph for determining if inter-cell movement is possible
///
/// For simplicity, it's assumed that intra-cell movement is always possible
use opengl_graphics::GlGraphics;
use shapes::Point;
use std::{
    fmt::{Debug, Error, Formatter},
    slice::Iter,
    sync::Arc,
};

use crate::{
    gameobject::Solidity, rendering::View, BoxEdge, Camera, HasBox, HasBoxMut, PhysBox,
    ScarabResult,
};

use super::{HasSolidity, AIR, SOLID};

#[derive(Debug, Clone)]
pub struct Field {
    cells: Vec<Arc<Cell>>,
    /// When edges[i][j] = Some(CellEdge) the edge between cells i and j
    /// is on the edge of i denoted by the variant of `CellEdge`
    edges: Vec<Vec<Option<BoxEdge>>>,
}

impl Field {
    /// Given a list of cells, construct their edges
    pub fn new(cells: Vec<Cell>) -> ScarabResult<Self> {
        let mut cnt = 0;
        let mut cells: Vec<Arc<Cell>> = cells
            .into_iter()
            .map(|mut c| {
                c.i = cnt;
                cnt += 1;
                Arc::new(c)
            })
            .collect();

        let mut edges = vec![vec![None; cells.len()]; cells.len()];

        Field::build_cells(&mut cells, &mut edges);

        // TODO: potential validation steps:
        // ensure that cells dont overlap (probably should be done before the set edges)
        //   it most likely can't be done exhaustively < O(n^2)
        //   but I don't know if that's the end of the world since this is only done
        //   on level loading

        Ok(Self { cells, edges })
    }

    fn build_cells(cells: &mut Vec<Arc<Cell>>, edges: &mut Vec<Vec<Option<BoxEdge>>>) -> () {
        // Find the bordering cells along the given edge,
        // and mark the appropriate graph edges
        // c: the current cell
        // test_pos: mut TileVec.
        //     Must be initialized to `TileVec(c.pos.0 + off_x, c.pos.1 + off_y)`
        //     where `off_x` is 0 when `direction` is 0 or c.size.0 otherwise and
        //     `off_y` is 0 when `direction is 1 or c.size.1 otherwise.
        //     This part is not pulled into the macro due to some complications
        //     with typing because TileVec is unsigned.
        // edge: A `BoxEdge` variant
        let mut cell_edges =
            |c: &Cell, test_pos: Point, edge: BoxEdge, neighbors: &mut Vec<Arc<Cell>>| {
                let mut test_pos = test_pos.clone();

                // We iterate in the direction is orthogonal to edge's axis
                while edge.get_normal_component_of(&test_pos)
                    < c.physbox.get_far_axis(edge.perpendicular_axis())
                {
                    // Find the cell at the current 'test_pos'. If it exists and
                    // is a valid edge (i.e. can be exited and then entered)
                    // then add it to neighbors
                    // Then set the new test pos to the far end of the neighbor
                    let new_normal_component = Field::cell_at_internal(&cells, test_pos)
                        .map_or_else(
                            || edge.get_normal_component_of(&test_pos) + 1.0,
                            |n| {
                                edges[c.i][n.i] = if c.solidity.exit_edge(edge)
                                    && n.solidity.enter_edge(edge.opposite())
                                {
                                    Some(edge)
                                } else {
                                    None
                                };
                                neighbors.push(Arc::clone(&n));
                                n.physbox.get_far_axis(edge.perpendicular_axis())
                            },
                        );

                    test_pos = match edge {
                        BoxEdge::Top | BoxEdge::Bottom => Point {
                            x: new_normal_component,
                            y: test_pos.y,
                        },
                        BoxEdge::Left | BoxEdge::Right => Point {
                            x: test_pos.x,
                            y: new_normal_component,
                        },
                    };
                }
            };

        // Initialize the neighbors and edges
        for i in 0..cells.len() {
            let mut cell = Arc::clone(&cells[i]);
            let physbox = cell.physbox;
            let mut test_pos;
            let mut top_neighbors = Vec::new();
            let mut left_neighbors = Vec::new();
            let mut bottom_neighbors = Vec::new();
            let mut right_neighbors = Vec::new();

            // Along the top edge
            test_pos = physbox.pos() - [0.0, 1.0];
            cell_edges(&cell, test_pos, BoxEdge::Top, &mut top_neighbors);

            // Along the left edge
            test_pos = physbox.pos() - [1.0, 0.0];
            cell_edges(&cell, test_pos, BoxEdge::Left, &mut left_neighbors);

            // Along the bottom edge
            test_pos = [physbox.left_x(), physbox.bottom_y()].into();
            cell_edges(&cell, test_pos, BoxEdge::Bottom, &mut bottom_neighbors);

            // Along the right edge
            test_pos = [physbox.right_x(), physbox.top_y()].into();
            cell_edges(&cell, test_pos, BoxEdge::Right, &mut right_neighbors);

            // Doing an unsafe `get_mut_unchecked` because we know this is the
            // only place that owns the cells at the moment
            unsafe {
                let c = Arc::get_mut_unchecked(&mut cell);
                c.top_neighbors = top_neighbors;
                c.left_neighbors = left_neighbors;
                c.bottom_neighbors = bottom_neighbors;
                c.right_neighbors = right_neighbors;
            }
        }
    }

    fn cell_at_internal(cells: &Vec<Arc<Cell>>, pos: Point) -> Option<Arc<Cell>> {
        for c in cells {
            if c.physbox.contains_pos(pos) {
                return Some(Arc::clone(c));
            }
        }
        None
    }

    pub fn cell_at(&self, pos: Point) -> Option<Arc<Cell>> {
        // The real challenge of this will be to try and do it in less than O(n)
        Field::cell_at_internal(&self.cells, pos)
    }

    // pub fn is_on_border(&self, pos: Vec2, size: TileVec) -> bool {
    //     todo!()
    // }
}

#[derive(Debug, Clone)]
pub struct FieldView {
    pub solid_view: CellView,
    pub air_view: CellView,
    pub default_view: CellView,
}

impl FieldView {
    fn view_for_cell(&self, cell: &Cell) -> &CellView {
        match cell.solidity {
            SOLID => &self.solid_view,
            AIR => &self.air_view,
            _ => &self.default_view,
        }
    }
}

impl View for FieldView {
    type Viewed = Field;

    fn render(
        &self,
        viewed: &Self::Viewed,
        camera: &Camera,
        ctx: Context,
        gl: &mut GlGraphics,
    ) -> ScarabResult<()> {
        for cell in &viewed.cells {
            let cell_view = self.view_for_cell(cell);
            cell_view.render(cell, camera, ctx, gl)?;
        }
        Ok(())
    }
}

pub struct Cell {
    /// This cell's index in the Field's `Vec<Cell>`
    i: usize,
    /// Defines how entities can move into/out of this cell
    solidity: Solidity,
    /// The upper left corner and width/height of the cell
    physbox: PhysBox,
    /// The Field indices of cells bordering this one along its top edge
    top_neighbors: Vec<Arc<Cell>>,
    /// The Field indices of cells bordering this one along its left edge
    left_neighbors: Vec<Arc<Cell>>,
    /// The Field indices of cells bordering this one along its bottom edge
    bottom_neighbors: Vec<Arc<Cell>>,
    /// The Field indices of cells bordering this one along its right edge
    right_neighbors: Vec<Arc<Cell>>,
}

impl Cell {
    pub fn new(solidity: Solidity, physbox: PhysBox) -> Self {
        Self {
            i: 0,
            solidity,
            physbox,
            top_neighbors: vec![],
            left_neighbors: vec![],
            bottom_neighbors: vec![],
            right_neighbors: vec![],
        }
    }

    fn edge_neighbors(&self, edge: BoxEdge) -> &Vec<Arc<Cell>> {
        match edge {
            BoxEdge::Top => &self.top_neighbors,
            BoxEdge::Left => &self.left_neighbors,
            BoxEdge::Bottom => &self.bottom_neighbors,
            BoxEdge::Right => &self.right_neighbors,
        }
    }

    /// Returns a list of the neighbors of this cell which the given physbox overlaps
    pub fn neighbors_overlapped(&self, physbox: &PhysBox) -> Vec<(BoxEdge, Arc<Cell>)> {
        self.physbox
            .edges_crossed_by(physbox)
            .into_iter()
            .flat_map(|edge| {
                self.edge_neighbors(edge)
                    .iter()
                    .map(move |cell| (edge, cell))
            })
            .filter(|(_edge, neighbor)| physbox.has_overlap(&neighbor.physbox))
            .map(|(edge, neighbor)| (edge, Arc::clone(neighbor)))
            .collect()
    }
}

// Manually implementing Debug for Cell b/c otherwise it would create an
// infinite loop between the neighbors.
// The main difference is showing neighbor's indexes rather than their full val.
impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Cell")
            .field("i", &self.i)
            .field("solidity", &self.solidity)
            .field("physbox", &self.physbox)
            .field(
                "top_neighbors",
                &self
                    .top_neighbors
                    .iter()
                    .map(|c| c.i)
                    .collect::<Vec<usize>>(),
            )
            .field(
                "left_neighbors",
                &self
                    .left_neighbors
                    .iter()
                    .map(|c| c.i)
                    .collect::<Vec<usize>>(),
            )
            .field(
                "bottom_neighbors",
                &self
                    .bottom_neighbors
                    .iter()
                    .map(|c| c.i)
                    .collect::<Vec<usize>>(),
            )
            .field(
                "right_neighbors",
                &self
                    .right_neighbors
                    .iter()
                    .map(|c| c.i)
                    .collect::<Vec<usize>>(),
            )
            .finish()
    }
}

impl HasBox for Cell {
    fn get_box(&self) -> &PhysBox {
        &self.physbox
    }
}

impl HasBoxMut for Cell {
    fn get_box_mut(&mut self) -> &mut PhysBox {
        &mut self.physbox
    }
}

impl HasSolidity for Cell {
    fn get_solidity(&self) -> &Solidity {
        &self.solidity
    }
}

// TODO: see if there's a way to not add extra impls for the Arc<Cell>
impl HasBox for Arc<Cell> {
    fn get_box(&self) -> &PhysBox {
        &self.physbox
    }
}

impl HasSolidity for Arc<Cell> {
    fn get_solidity(&self) -> &Solidity {
        &self.solidity
    }
}

pub struct CellNeighbors {
    pub top: Vec<Arc<Cell>>,
    pub left: Vec<Arc<Cell>>,
    pub bottom: Vec<Arc<Cell>>,
    pub right: Vec<Arc<Cell>>,
}

impl CellNeighbors {
    pub fn new() -> Self {
        Self {
            top: Vec::new(),
            left: Vec::new(),
            bottom: Vec::new(),
            right: Vec::new(),
        }
    }

    pub fn add_neighbor(&mut self, neighbor: Arc<Cell>, edge: BoxEdge) {
        match edge {
            BoxEdge::Top => self.top.push(neighbor),
            BoxEdge::Left => self.left.push(neighbor),
            BoxEdge::Bottom => self.bottom.push(neighbor),
            BoxEdge::Right => self.right.push(neighbor),
        }
    }

    pub fn get_neighbors(&self, edge: BoxEdge) -> &Vec<Arc<Cell>> {
        match edge {
            BoxEdge::Top => &self.top,
            BoxEdge::Left => &self.left,
            BoxEdge::Bottom => &self.bottom,
            BoxEdge::Right => &self.right,
        }
    }

    pub fn iter(&self) -> CellNeighborsIter {
        CellNeighborsIter {
            current_edge: BoxEdge::iter(),
            inner: &self,
        }
    }
}

impl From<Vec<(BoxEdge, Arc<Cell>)>> for CellNeighbors {
    fn from(val: Vec<(BoxEdge, Arc<Cell>)>) -> Self {
        let mut neighbors = CellNeighbors::new();

        for (edge, cell) in val {
            neighbors.add_neighbor(cell, edge);
        }

        neighbors
    }
}

pub struct CellNeighborsIter<'a> {
    current_edge: Iter<'static, BoxEdge>,
    inner: &'a CellNeighbors,
}

impl<'a> Iterator for CellNeighborsIter<'a> {
    type Item = (BoxEdge, &'a Vec<Arc<Cell>>);

    // TODO: there's a chance for slight optimizations here to avoid
    // the cloning of the vec.
    fn next(&mut self) -> Option<Self::Item> {
        self.current_edge
            .next()
            .map(|edge| (edge.to_owned(), self.inner.get_neighbors(*edge)))
    }
}

#[derive(Debug, Clone)]
pub struct CellView {
    pub color: Color,
}

impl View for CellView {
    type Viewed = Cell;

    fn render(
        &self,
        viewed: &Self::Viewed,
        camera: &Camera,
        ctx: Context,
        gl: &mut GlGraphics,
    ) -> ScarabResult<()> {
        if let Some((transform, rect)) = camera.box_renderables(viewed.physbox, ctx) {
            graphics::rectangle(self.color, rect, transform, gl);
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {}
