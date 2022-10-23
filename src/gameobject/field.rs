/// A field is a graph of rectangles (cells) that each have a (backup) rendering
/// component and a collision (or non-collision) component
///
/// The cells have a solidity field (defined in mod.rs) which dictates
/// how standard entities can move between cells.
/// In the construction of a gamefield from the given cells, their solidity is
/// used to construct a graph for determining if inter-cell movement is possible
///
/// For simplicity, it's assumed that intra-cell movement is always possible
use std::sync::Arc;

use opengl_graphics::GlGraphics;

use crate::{
    gameobject::Solidity, rendering::Renderable, Camera, Color, HasBox, HasBoxMut, PhysBox,
    ScarabResult, TileVec, VecNum,
};

use super::HasSolidity;

#[derive(Debug, Clone)]
pub struct Field {
    cells: Vec<Arc<Cell>>,
    edges: Vec<Vec<Option<CellEdge>>>,
}

impl Field {
    /// Given a list of cells, construct their edges
    pub fn new(cells: Vec<Cell>) -> Self {
        let mut cnt = 0;
        let cells = cells
            .into_iter()
            .map(|mut c| {
                c.i = cnt;
                cnt += 1;
                Arc::new(c)
            })
            .collect::<Vec<Arc<Cell>>>();
        let mut edges = vec![vec![None; cells.len()]; cells.len()];

        for c in &cells {
            Field::set_edges_internal(c, &cells, &mut edges);
        }

        // TODO: potential validation steps:
        // ensure that cells dont overlap (probably should be done before the set edges)
        //   it most likely can't be done exhaustively < O(n^2)
        //   but I don't know if that's the end of the world since this is only done
        //   on level loading

        Self { cells, edges }
    }

    fn set_edges_internal(
        c: &Arc<Cell>,
        cells: &Vec<Arc<Cell>>,
        edges: &mut Vec<Vec<Option<CellEdge>>>,
    ) {
        /// Find the bordering cells along the given edge,
        /// and mark the appropriate graph edges
        /// c: the current cell
        /// cells: &Vec<Arc<Cell>>
        /// edges: &mut Vec<Vec<bool>>
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
                $edge:expr
            ) => {
                while $test_pos.$direction
                    < $c.physbox.pos().$direction + $c.physbox.size().$direction
                {
                    let new_pos = Field::cell_at_internal($cells, $test_pos).map_or_else(
                        || $test_pos.$direction + 1,
                        |n| {
                            $edges[c.i][n.i] =
                                if $c.solidity.exit_top() && n.solidity.enter_bottom() {
                                    Some($edge)
                                } else {
                                    None
                                };
                            n.physbox.pos().$direction + n.physbox.size().$direction
                        },
                    );
                    $test_pos.$direction = new_pos;
                }
            };
        }

        let physbox = c.physbox;

        // Along the top edge
        if physbox.pos().y() != 0 {
            let mut test_pos = TileVec::new(physbox.pos().x(), physbox.pos().y() - 1);
            cell_edges!(c, cells, edges, test_pos, x, y, CellEdge::Top);
        }

        // Along the left edge
        if physbox.pos().x() != 0 {
            let mut test_pos = TileVec::new(physbox.pos().x() - 1, physbox.pos().y());
            cell_edges!(c, cells, edges, test_pos, y, x, CellEdge::Left);
        }

        // Along the bottom edge
        let mut test_pos = TileVec::new(physbox.pos().x(), physbox.pos().y() + physbox.size().y());
        cell_edges!(c, cells, edges, test_pos, x, y, CellEdge::Bottom);

        // Along the right edge
        test_pos = TileVec::new(physbox.pos().x() + physbox.size().x(), physbox.pos().y());
        cell_edges!(c, cells, edges, test_pos, y, x, CellEdge::Right);
    }

    fn cell_at_internal<N: VecNum>(cells: &Vec<Arc<Cell>>, pos: TileVec<N>) -> Option<Arc<Cell>> {
        // The real challenge of this will be to try and do it in less than O(n)
        for c in cells {
            if c.physbox.convert_n().contains_pos(pos) {
                return Some(c.clone());
            }
        }
        None
    }

    pub fn cell_at<N: VecNum>(&self, pos: TileVec<N>) -> Option<Arc<Cell>> {
        Field::cell_at_internal(&self.cells, pos)
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

// impl Renderable for Field {
//     fn render(
//         &self,
//         camera: Camera,
//         ctx: graphics::Context,
//         gl: &mut GlGraphics,
//     ) -> ScarabResult<()> {
//         for c in &self.cells {
//             c.render(camera, ctx, gl)?;
//         }
//         Ok(())
//     }
// }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CellEdge {
    Top,
    Left,
    Bottom,
    Right,
}

/// TODO: Cell should not be publicly constructible with raw vals
#[derive(Debug)]
pub struct Cell {
    /// This cell's index in the Vec<Cell>
    pub i: usize,
    /// Defines how entities can move into/out of this cell
    pub solidity: Solidity,
    /// Debug rendering color
    pub color: Color,
    /// The upper left corner and width/height of the cell
    pub physbox: PhysBox<u32>,
    // TODO:
    // have a field that gives the indices of the cells at each edge
}

impl Cell {}

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
