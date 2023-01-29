use std::{fmt::Debug, sync::Arc};

use crate::{
    gameobject::{
        field::{Cell, CellNeighbors},
        HasHealth, HasSolidity, Health, Solidity, SOLID,
    },
    rendering::View,
    Camera, HasBox, HasBoxMut, PhysBox, ScarabError, ScarabResult, Velocity,
};

mod entity_controls;
pub use entity_controls::EntityControls;
pub mod registry;
use graphics::{
    types::{Color, Scalar},
    Context,
};
use opengl_graphics::GlGraphics;

use super::Field;

pub trait HasEntity<'a, 'b: 'a> {
    fn get_entity(&'b self) -> &'a Entity;

    fn get_entity_mut(&'b mut self) -> &'a mut Entity;
}

#[derive(Debug)]
pub struct Entity {
    velocity: Velocity,
    max_velocity: Scalar,
    physbox: PhysBox,
    health: Health,
    solidity: Solidity,
}

impl Entity {
    pub fn new() -> ScarabResult<Self> {
        Ok(Self {
            velocity: [0.0, 0.0].into(),
            max_velocity: 1.0,
            physbox: PhysBox::new([0.0, 0.0, 1.0, 1.0].into())?,
            health: Health::new(10),
            solidity: SOLID,
        })
    }
    /// Sets the entity's velocity in terms of its maximum velocity
    pub fn set_velocity(&mut self, velocity: Velocity) {
        self.velocity = velocity.normalize() * self.max_velocity;
    }

    pub fn set_max_velocity(&mut self, max_velocity: Scalar) -> ScarabResult<()> {
        if max_velocity <= 0.0 {
            return Err(ScarabError::RawString(
                "Maximum velocity must be positive".to_string(),
            ));
        }
        self.max_velocity = max_velocity;

        Ok(())
    }

    /// Get the position of the entity after its next movement assuming no collisions
    pub fn get_projected_box(&self) -> PhysBox {
        let mut physbox = self.physbox.clone();
        physbox.set_pos(physbox.pos() + self.velocity);
        physbox
    }

    /// Returns a callback function for resolving entity-entity collisions
    pub fn game_tick(&mut self, field: &Field, dt: f64) -> ScarabResult<()> {
        self.try_move(field, dt)
    }

    fn try_move(&mut self, field: &Field, dt: f64) -> ScarabResult<()> {
        if self.velocity == [0.0, 0.0].into() {
            return Ok(());
        }

        // TODO: having to recalculate the current cell every time will get time intensive
        // Should create a new function to take into account the old current cell and its neighbors
        // at the very least only going through those. Even more so, we can add the edges that were
        // crossed.
        let (cell, overlaps): (_, Vec<Arc<Cell>>) = field
            .cell_at(self.physbox.pos())
            .map(|c| {
                let overlaps = c
                    .neighbors_overlapped(&self.physbox)
                    .iter()
                    .map(|(_edge, c)| Arc::clone(c))
                    .collect();
                (c, overlaps)
            })
            .ok_or_else(|| ScarabError::RawString("can't find current cell".to_string()))?;

        let new_pos = self.physbox.pos() + self.velocity * dt;
        let mut new_box = self.physbox.clone();
        new_box.set_pos(new_pos);

        // Cell Based collisions
        if !new_box.is_fully_contained_by(&cell.get_box()) {
            let mut apply_movement_reductions = |c: &Arc<Cell>| -> ScarabResult<()> {
                let neighbors = CellNeighbors::from(c.neighbors_overlapped(&new_box));
                for (edge, neighbors) in neighbors.iter() {
                    for neighbor in neighbors {
                        if (!c.get_solidity().exit_edge(edge)
                            || !neighbor.get_solidity().enter_edge(edge))
                            && self.velocity.is_reduced_by_edge(edge)
                        {
                            new_box.set_touching_edge(&c.get_box(), edge);
                        }
                    }

                    // Don't care about the solidity when we're leaving and not
                    // entering into a new neighbor because this can lead to tricky behavoir
                    // because at present, movement is not defined when the entity is not
                    // fully contained by some number of cells
                    if neighbors.len() == 0
                        && self.velocity.is_reduced_by_edge(edge)
                        && c.get_box().is_edge_crossed_by(&new_box, edge)
                    {
                        new_box.set_touching_edge(&c.get_box(), edge);
                    }
                }

                Ok(())
            };

            apply_movement_reductions(&cell)?;

            for o in overlaps {
                apply_movement_reductions(&o)?;
            }
        }

        self.physbox = new_box;

        // TODO: switch to a separate "resolve entity collisions step"
        // doing these collated will definite cause problems as the number
        // of entities increases
        Ok(())
    }
}

impl HasBox for Entity {
    fn get_box(&self) -> &PhysBox {
        &self.physbox
    }
}

impl HasBoxMut for Entity {
    fn get_box_mut(&mut self) -> &mut PhysBox {
        &mut self.physbox
    }
}

impl HasHealth for Entity {
    fn get_health(&self) -> &Health {
        &self.health
    }
}

impl HasSolidity for Entity {
    fn get_solidity(&self) -> &Solidity {
        &self.solidity
    }
}

#[derive(Debug, Clone)]
pub struct EntityView {
    pub color: Color,
}

impl View for EntityView {
    type Viewed = Entity;

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

impl View for &EntityView {
    type Viewed = Entity;

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
