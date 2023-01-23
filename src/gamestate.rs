use std::{
    fmt::Debug,
    ptr,
    sync::{Arc, RwLock},
};

use graphics::Context;
use opengl_graphics::GlGraphics;

use crate::{
    control::UpdateChannel,
    gameobject::{
        entity::{Entity, EntityView},
        field::{Field, FieldView},
        HasSolidity,
    },
    playercontroller::InputController,
    rendering::View,
    Camera, HasBox, PhysBox, ScarabResult,
};

#[derive(Debug)]
pub struct Gamestate<'a> {
    field: Arc<Field>,
    field_view: FieldView,
    entities: Vec<(Entity, &'a EntityView)>,
    input_controllers: Vec<InputController>,
}

impl<'a> Gamestate<'a> {
    pub fn new(field: Field, field_view: FieldView) -> Self {
        Self {
            field: Arc::new(field),
            field_view,
            entities: vec![],
            input_controllers: vec![],
        }
    }

    pub fn render(&self, camera: &Camera, ctx: Context, gl: &mut GlGraphics) -> ScarabResult<()> {
        self.field_view.render(&self.field, &camera, ctx, gl)?;

        for (entity, view) in &self.entities {
            view.render(entity, camera, ctx, gl)?;
        }
        Ok(())
    }

    pub fn add_entity(
        arc: &Arc<RwLock<Self>>,
        entity: Entity,
        view: &'a EntityView,
    ) -> ScarabResult<()> {
        let mut state = arc.write().unwrap();
        state.entities.push((entity, view));
        Ok(())
    }

    pub fn get_field(&self) -> &Field {
        &self.field
    }

    pub fn add_input_controller(&mut self, controller: InputController) {
        self.input_controllers.push(controller);
    }

    pub fn player(&self) -> Option<&Entity> {
        self.entities.get(0).map(|(e, _)| e)
    }

    pub fn player_mut(&mut self) -> Option<&mut Entity> {
        self.entities.get_mut(0).map(|(e, _)| e)
    }

    pub fn update(&mut self, dt: f64) -> ScarabResult<()> {
        // Do not collate the channel reads and game ticks so that we can
        // be more confident in the entities projected positions
        // for entity-entity collisions
        for i in 0..self.entities.len() {
            self.entities[i].0.exhaust_channel()?;
        }

        unsafe {
            let entities_ptr = self.entities.as_mut_ptr();
            for i in 0..self.entities.len() {
                if let Some(e) = entities_ptr.add(i).as_mut() {
                    e.0.game_tick(self, dt)?;
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

    pub fn overlapping_entity_boxes(&self, entity: &Entity, physbox: &PhysBox) -> Vec<PhysBox> {
        self.entities
            .iter()
            .filter(|e| !ptr::eq(&e.0, entity))
            .filter(|e| e.0.get_solidity().has_solidity())
            .filter_map(|(e, _v)| {
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

// TODO: how to renderrrrr a game state in the different layers?
