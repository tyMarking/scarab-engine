use std::{
    fmt::Debug,
    ptr,
    sync::{Arc, RwLock},
};

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::UpdateArgs;

use crate::{
    control::{DoUpdate, UpdateChannel},
    gameobject::{
        entity::{Entity, EntityModel},
        field::Field,
        HasSolidity,
    },
    playercontroller::InputController,
    Camera, HasBox, PhysBox, ScarabResult, VecNum,
};

pub struct Gamestate<N: VecNum> {
    field: Arc<Field>,
    entities: Vec<Entity<N>>,
    input_controllers: Vec<InputController>,
}

impl<N: VecNum> Gamestate<N> {
    pub fn new(field: Field) -> Self {
        Self {
            field: Arc::new(field),
            entities: vec![],
            input_controllers: vec![],
        }
    }

    pub fn render(&self, camera: &Camera, ctx: Context, gl: &mut GlGraphics) -> ScarabResult<()> {
        self.field.render(&camera, ctx, gl)?;

        camera.render_boxes(&self.entities, ctx, gl)?;

        Ok(())
    }

    pub fn add_entity(arc: &Arc<RwLock<Self>>, mut entity: Entity<N>) -> ScarabResult<()> {
        entity.set_gamestate(Arc::clone(arc));
        let mut state = arc.write().unwrap();
        state.entities.push(entity);
        Ok(())
    }

    pub fn get_field(&self) -> &Field {
        &self.field
    }

    pub fn add_input_controller(&mut self, controller: InputController) {
        self.input_controllers.push(controller);
    }

    pub fn player(&self) -> Option<&Entity<N>> {
        self.entities.get(0)
    }

    pub fn player_mut(&mut self) -> Option<&mut Entity<N>> {
        self.entities.get_mut(0)
    }

    pub fn update(&mut self, dt: f64) -> ScarabResult<()> {
        // Do not collate the channel reads and game ticks so that we can
        // be more confident in the entities projected positions
        // for entity-entity collisions
        for i in 0..self.entities.len() {
            self.entities[i].exhaust_channel()?;
        }

        unsafe {
            let entities_ptr = self.entities.as_mut_ptr();
            for i in 0..self.entities.len() {
                if let Some(e) = entities_ptr.add(i).as_mut() {
                    e.game_tick(self, dt)?;
                }
            }
        }

        // self.resolve_entity_collisions()?;

        Ok(())
    }

    pub fn input_event(&mut self, input: piston::Input) -> ScarabResult<()> {
        for i in 0..self.input_controllers.len() {
            let _ = self.input_controllers[i].input_event(&input);
        }

        Ok(())
    }

    // fn resolve_entity_collisions(&mut self) -> ScarabResult<()> {
    //     // Entity Based collisions are lazy in terms of solidity.
    //     // As long as both have any solidity the collision will happen
    //     // if self.solidity.has_solidity() {
    //     //     for physbox in gamestate.overlapping_entity_boxes(self, &new_box) {
    //     //         new_box.shift_to_nonoverlapping(&physbox);
    //     //     }
    //     // }
    //     Ok(())
    // }

    pub fn overlapping_entity_boxes(
        &self,
        entity: &EntityModel<N>,
        physbox: &PhysBox<N>,
    ) -> Vec<PhysBox<N>> {
        self.entities
            .iter()
            .filter(|e| !ptr::eq(e.get_model(), entity))
            .filter(|e| e.get_solidity().has_solidity())
            .filter_map(|e| {
                let projected = e.get_projected_box();
                if physbox.has_overlap(&projected) {
                    Some(projected)
                } else if physbox.has_overlap(e.get_box()) {
                    Some(*e.get_box())
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<N: VecNum> Debug for Gamestate<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Gamestate")
            .field("field", &self.field)
            .field("entities", &self.entities.len())
            .field("input_controllers", &self.input_controllers.len())
            .finish()
    }
}

// TODO: how to renderrrrr a game state in the different layers?
