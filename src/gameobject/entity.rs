use std::sync::Arc;

use crate::{
    gameobject::{
        field::{Cell, Field},
        HasHealth, HasSolidity, Health, Solidity, SOLID,
    },
    rendering::Renderable,
    Color, HasBox, HasBoxMut, PhysBox, ScarabError, ScarabResult, TileVec, VecNum,
};

#[derive(Debug)]
pub struct Entity<N: VecNum> {
    physbox: PhysBox<N>,
    color: Color,
    health: Health,
    solidity: Solidity,
    current_cell: Option<Arc<Cell>>,
    field: Option<Arc<Field>>,
}

impl<N: VecNum> Entity<N> {
    pub fn set_field(&mut self, field: Arc<Field>) {
        self.field = Some(field);
    }

    pub fn try_move(&mut self, direction: TileVec<N>) -> ScarabResult<()> {
        let f = self
            .field
            .as_ref()
            .ok_or_else(|| ScarabError::RawString("can't move without a field set".to_string()))?;

        if !self.current_cell.is_some() {
            self.current_cell = f.cell_at(self.physbox.pos())?;
        }
        let cell = self
            .current_cell
            .as_ref()
            .ok_or_else(|| ScarabError::RawString("can't find current cell".to_string()))?;

        let new_pos = self.physbox.pos() + direction;
        let mut new_box = self.physbox.clone();
        new_box.set_pos(new_pos)?;

        if new_box.is_fully_contained_by(&cell.get_box().convert_n()) {
            // We can just set the new position
            self.physbox.set_pos(new_pos)?;
        } else {
            let neighbors = cell.neighbors_overlapped(&new_box)?;
            println!("{neighbors:?}");
            for (edge, neighbor) in &neighbors {
                if !cell.get_solidity().exit_edge(*edge)
                    || !neighbor.get_solidity().enter_edge(*edge)
                {
                    new_box.set_touching_edge(&cell.get_box().convert_n(), *edge)?;
                }
            }

            if neighbors.len() == 0 {
                let edges = cell.get_box().convert_n().edges_crossed_by(&new_box);
                for edge in edges {
                    if !cell.get_solidity().exit_edge(edge) {
                        new_box.set_touching_edge(&cell.get_box().convert_n(), edge)?;
                    }
                }
            }
            self.physbox = new_box;
            // TODO: having to recalculate the current cell every time will get time intensive
            // Should create a new function to take into account the old current cell and its neighbors
            // at the very least only going through those. Even more so, we can add the edges that were
            // crossed.
            if !cell.get_box().convert_n().contains_pos(new_box.pos()) {
                self.current_cell = f.cell_at(new_box.pos())?;
            }
        }

        Ok(())
    }

    pub fn current_cell(&self) -> Option<&Arc<Cell>> {
        self.current_cell.as_ref()
    }
}

impl Entity<f64> {
    pub fn new_def() -> ScarabResult<Self> {
        Ok(Self {
            physbox: PhysBox::new((0.0, 0.0), (1.0, 1.0))?,
            color: [0.0, 1.0, 1.0, 1.0],
            health: Health::new(10),
            solidity: SOLID,
            current_cell: None,
            field: None,
        })
    }
}

impl<N: VecNum> HasBox<N> for Entity<N> {
    fn get_box(&self) -> &PhysBox<N> {
        &self.physbox
    }
}

impl<N: VecNum> HasBoxMut<N> for Entity<N> {
    fn get_box_mut(&mut self) -> &mut PhysBox<N> {
        &mut self.physbox
    }
}

impl<N: VecNum> HasHealth for Entity<N> {
    fn get_health(&self) -> &Health {
        &self.health
    }
}

impl<N: VecNum> HasSolidity for Entity<N> {
    fn get_solidity(&self) -> &Solidity {
        &self.solidity
    }
}

impl<N: VecNum> Renderable for Entity<N> {
    fn color(&self) -> &Color {
        &self.color
    }
}
