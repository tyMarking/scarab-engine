use std::sync::{
    mpsc::{self, Receiver, Sender, TryRecvError},
    Arc,
};

use crate::{
    control::UpdateChannel,
    gameobject::{
        field::{Cell, Field},
        HasHealth, HasSolidity, Health, Solidity, SOLID,
    },
    rendering::Renderable,
    utils::normalize,
    Color, HasBox, HasBoxMut, PhysBox, ScarabError, ScarabResult, TileVec, VecNum,
};

mod entity_controls;
pub use entity_controls::EntityControls;

#[derive(Debug)]
pub struct Entity<N: VecNum> {
    model: EntityModel<N>,
    view: Color,
    channel: (Sender<EntityControls>, Receiver<EntityControls>),
}

impl<N: VecNum> Entity<N> {
    pub fn set_field(&mut self, field: Arc<Field>) {
        self.model.field = Some(field);
    }

    /// Sets the entity's velocity in terms of its maximum velocity
    pub fn set_velocity(&mut self, vel: [f64; 2]) {
        self.model.velocity = TileVec::from_f64_unchecked(TileVec::from(normalize(vel)))
            .scale(self.model.max_velocity.into());
    }

    pub fn set_max_velocity(&mut self, v: N) -> ScarabResult<()> {
        if v.into() <= 0.0 {
            return Err(ScarabError::RawString(
                "Maximum velocity must be positive".to_string(),
            ));
        }
        self.model.max_velocity = v;

        Ok(())
    }
}

impl<N: VecNum> Renderable for Entity<N> {
    fn color(&self) -> &Color {
        &self.view
    }
}

impl Entity<f64> {
    pub fn new_def() -> ScarabResult<Self> {
        Ok(Self {
            model: EntityModel {
                velocity: TileVec::new(0.0, 0.0),
                max_velocity: 1.0,
                physbox: PhysBox::new((0.0, 0.0), (1.0, 1.0))?,
                health: Health::new(10),
                solidity: SOLID,
                current_cell: None,
                field: None,
            },
            view: [0.0, 1.0, 1.0, 1.0],
            channel: mpsc::channel(),
        })
    }
}

impl<N: VecNum> UpdateChannel<EntityControls> for Entity<N> {
    fn game_tick(&mut self, dt: f64) -> ScarabResult<()> {
        self.model.update(dt)
    }

    fn get_sender(&self) -> Sender<EntityControls> {
        self.channel.0.clone()
    }

    fn consume_channel(&mut self) -> Option<Result<(), TryRecvError>> {
        let res = self.channel.1.try_recv().map_or_else(
            |err| match err {
                TryRecvError::Empty => None,
                other => Some(Err(other)),
            },
            |r| Some(Ok(r)),
        )?;

        if let Err(err) = res {
            return Some(Err(err));
        }

        let cmd = res.unwrap();
        match cmd {
            EntityControls::SetMovement(v) => self.set_velocity(v),
            EntityControls::Nop => {}
        }

        Some(Ok(()))
    }
}

impl<N: VecNum> HasBox<N> for Entity<N> {
    fn get_box(&self) -> &PhysBox<N> {
        &self.model.physbox
    }
}

impl<N: VecNum> HasBoxMut<N> for Entity<N> {
    fn get_box_mut(&mut self) -> &mut PhysBox<N> {
        &mut self.model.physbox
    }
}

impl<N: VecNum> HasHealth for Entity<N> {
    fn get_health(&self) -> &Health {
        &self.model.health
    }
}

impl<N: VecNum> HasSolidity for Entity<N> {
    fn get_solidity(&self) -> &Solidity {
        &self.model.solidity
    }
}

#[derive(Debug)]
struct EntityModel<N: VecNum> {
    /// Velocity of the entity in tiles/second
    velocity: TileVec<N>,
    max_velocity: N,
    physbox: PhysBox<N>,
    health: Health,
    solidity: Solidity,
    current_cell: Option<(Arc<Cell>, Vec<Arc<Cell>>)>,
    field: Option<Arc<Field>>,
}

impl<N: VecNum> EntityModel<N> {
    fn current_cell(&self) -> Option<&Arc<Cell>> {
        self.current_cell.as_ref().map(|(c, o)| c)
    }

    fn update(&mut self, dt: f64) -> ScarabResult<()> {
        self.try_move(dt)?;

        Ok(())
    }

    fn try_move(&mut self, dt: f64) -> ScarabResult<()> {
        let f = self
            .field
            .as_ref()
            .ok_or_else(|| ScarabError::RawString("can't move without a field set".to_string()))?;

        if self.velocity == TileVec::zero() {
            return Ok(());
        }

        // TODO: having to recalculate the current cell every time will get time intensive
        // Should create a new function to take into account the old current cell and its neighbors
        // at the very least only going through those. Even more so, we can add the edges that were
        // crossed.
        if !self.current_cell.is_some() {
            self.current_cell = f
                .cell_at(self.physbox.pos())?
                .map(|c| {
                    let overlaps = c
                        .neighbors_overlapped(&self.physbox)?
                        .iter()
                        .map(|(edge, c)| Arc::clone(c))
                        .collect();
                    Result::<_, ScarabError>::Ok((c, overlaps))
                })
                .transpose()?;
        }
        let (cell, overlaps) = self
            .current_cell
            .as_ref()
            .ok_or_else(|| ScarabError::RawString("can't find current cell".to_string()))?;

        let new_pos = self.physbox.pos() + self.velocity.scale(dt);
        let mut new_box = self.physbox.clone();
        new_box.set_pos(new_pos)?;

        if new_box.is_fully_contained_by(&cell.get_box().convert_n()) {
            // We can just set the new position
            self.physbox.set_pos(new_pos)?;
        } else {
            let mut apply_movement_reductions = |c: &Arc<Cell>| -> ScarabResult<()> {
                let neighbors = c.neighbors_overlapped(&new_box)?;
                // println!("NEIGHBORS: {neighbors:?}");
                for (edge, neighbor) in &neighbors {
                    if (!c.get_solidity().exit_edge(*edge)
                        || !neighbor.get_solidity().enter_edge(*edge))
                        && self.velocity.is_reduced_by_edge(edge)
                    {
                        new_box.set_touching_edge(&c.get_box().convert_n(), *edge)?;
                    }
                }

                if neighbors.len() == 0 {
                    let edges = c.get_box().convert_n().edges_crossed_by(&new_box);
                    for edge in edges {
                        if !c.get_solidity().exit_edge(edge)
                            && self.velocity.is_reduced_by_edge(&edge)
                        {
                            new_box.set_touching_edge(&c.get_box().convert_n(), edge)?;
                        }
                    }
                }

                Ok(())
            };

            apply_movement_reductions(cell)?;

            for o in overlaps {
                apply_movement_reductions(o)?;
            }

            self.physbox = new_box;
        }

        self.current_cell = None;

        Ok(())
    }
}

impl<N: VecNum> HasBox<N> for EntityModel<N> {
    fn get_box(&self) -> &PhysBox<N> {
        &self.physbox
    }
}

impl<N: VecNum> HasHealth for EntityModel<N> {
    fn get_health(&self) -> &Health {
        &self.health
    }
}

impl<N: VecNum> HasSolidity for EntityModel<N> {
    fn get_solidity(&self) -> &Solidity {
        &self.solidity
    }
}
