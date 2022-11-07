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
use std::{
    fmt::{Debug, Error, Formatter},
    sync::Arc,
};

use crate::{
    gameobject::Solidity, rendering::Renderable, BoxEdge, Camera, Color, HasBox, HasBoxMut,
    PhysBox, ScarabError, ScarabResult, TileVec, VecNum,
};

use super::HasSolidity;

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
        /// Find the bordering cells along the given edge,
        /// and mark the appropriate graph edges
        /// c: the current cell
        /// cells: the vec of cells
        /// edges: the edges to be initialized
        /// test_pos: mut TileVec.
        ///     Must be initialized to `TileVec(c.pos.0 + off_x, c.pos.1 + off_y)`
        ///     where `off_x` is 0 when `direction` is 0 or c.size.0 otherwise and
        ///     `off_y` is 0 when `direction is 1 or c.size.1 otherwise.
        ///     This part is not pulled into the macro due to some complications
        ///     with typing because TileVec is unsigned.
        /// direction: x or y
        /// ortho_direction: y or x, must be the opposite of `direction`
        /// edge: A `CellEdge` variant
        macro_rules! cell_edges {
            ($c:ident,
                $cells:ident,
                $edges:ident,
                $test_pos:ident,
                $direction:tt,
                $ortho_direction:tt,
                $edge:expr,
                $neighbors:tt
            ) => {
                while $test_pos.$direction
                    < $c.physbox.pos().$direction + $c.physbox.size().$direction
                {
                    let new_pos = Field::cell_at_internal(&$cells, $test_pos).map_or_else(
                        || $test_pos.$direction + 1,
                        |n| {
                            edges[$c.i][n.i] =
                                if $c.solidity.exit_top() && n.solidity.enter_bottom() {
                                    Some($edge)
                                } else {
                                    None
                                };
                            $neighbors.push(Arc::clone(&n));
                            n.physbox.pos().$direction + n.physbox.size().$direction
                        },
                    );
                    $test_pos.$direction = new_pos;
                }
            };
        }

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
            if physbox.pos().y() != 0 {
                test_pos = TileVec::new(physbox.pos().x(), physbox.pos().y() - 1);

                cell_edges!(
                    cell,
                    cells,
                    edges,
                    test_pos,
                    x,
                    y,
                    BoxEdge::Top,
                    top_neighbors
                );
            }

            // Along the left edge
            if physbox.pos().x() != 0 {
                test_pos = TileVec::new(physbox.pos().x() - 1, physbox.pos().y());
                cell_edges!(
                    cell,
                    cells,
                    edges,
                    test_pos,
                    y,
                    x,
                    BoxEdge::Left,
                    left_neighbors
                );
            }

            // Along the bottom edge
            test_pos = TileVec::new(physbox.pos().x(), physbox.pos().y() + physbox.size().y());
            cell_edges!(
                cell,
                cells,
                edges,
                test_pos,
                x,
                y,
                BoxEdge::Bottom,
                bottom_neighbors
            );

            // Along the right edge
            test_pos = TileVec::new(physbox.pos().x() + physbox.size().x(), physbox.pos().y());
            cell_edges!(
                cell,
                cells,
                edges,
                test_pos,
                y,
                x,
                BoxEdge::Right,
                right_neighbors
            );

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

    fn validate_pos<N: VecNum>(pos: TileVec<N>) -> ScarabResult<()> {
        if pos.x().into() < 0.0 || pos.y().into() < 0.0 {
            return Err(ScarabError::FieldPosition);
        }
        Ok(())
    }

    fn cell_at_internal<N: VecNum>(cells: &Vec<Arc<Cell>>, pos: TileVec<N>) -> Option<Arc<Cell>> {
        for c in cells {
            if c.physbox.convert_n().contains_pos(pos) {
                return Some(Arc::clone(c));
            }
        }
        None
    }

    pub fn cell_at<N: VecNum>(&self, pos: TileVec<N>) -> ScarabResult<Option<Arc<Cell>>> {
        Field::validate_pos(pos)?;
        // The real challenge of this will be to try and do it in less than O(n)
        Ok(Field::cell_at_internal(&self.cells, pos))
    }

    // pub fn is_on_border(&self, pos: Vec2, size: TileVec) -> bool {
    //     todo!()
    // }

    pub fn render(
        &self,
        camera: &Camera,
        ctx: graphics::Context,
        gl: &mut GlGraphics,
    ) -> ScarabResult<()> {
        camera.render_boxes(&self.cells, ctx, gl)
    }
}

pub struct Cell {
    /// This cell's index in the Field's `Vec<Cell>`
    i: usize,
    /// Defines how entities can move into/out of this cell
    solidity: Solidity,
    /// Debug rendering color
    color: Color,
    /// The upper left corner and width/height of the cell
    physbox: PhysBox<u32>,
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
    pub fn new(solidity: Solidity, color: Color, physbox: PhysBox<u32>) -> Self {
        Self {
            i: 0,
            solidity,
            color,
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
    pub fn neighbors_overlapped<N: VecNum>(
        &self,
        physbox: &PhysBox<N>,
    ) -> ScarabResult<Vec<(BoxEdge, Arc<Cell>)>> {
        Ok(self
            .physbox
            .convert_n()
            .edges_crossed_by(physbox)
            .into_iter()
            .flat_map(|edge| {
                self.edge_neighbors(edge)
                    .iter()
                    .map(move |cell| (edge, cell))
            })
            .filter(|(_edge, neighbor)| physbox.has_overlap(&neighbor.physbox.convert_n()))
            .map(|(edge, neighbor)| (edge, Arc::clone(neighbor)))
            .collect())
    }
}

// Manually implementing Debug for Cell b/c otherwise it would create an
// infinite loop between the neighbors
impl Debug for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.debug_struct("Cell")
            .field("i", &self.i)
            .field("solidity", &self.solidity)
            .field("color", &self.color)
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

impl HasBox<u32> for Cell {
    fn get_box(&self) -> &PhysBox<u32> {
        &self.physbox
    }
}

impl HasBoxMut<u32> for Cell {
    fn get_box_mut(&mut self) -> &mut PhysBox<u32> {
        &mut self.physbox
    }
}

impl Renderable for Cell {
    fn color(&self) -> &Color {
        &self.color
    }
}

impl HasSolidity for Cell {
    fn get_solidity(&self) -> &Solidity {
        &self.solidity
    }
}

impl HasBox<u32> for Arc<Cell> {
    fn get_box(&self) -> &PhysBox<u32> {
        &self.physbox
    }
}

impl Renderable for Arc<Cell> {
    fn color(&self) -> &Color {
        &self.color
    }
}

impl HasSolidity for Arc<Cell> {
    fn get_solidity(&self) -> &Solidity {
        &self.solidity
    }
}

#[cfg(test)]
mod test {}
