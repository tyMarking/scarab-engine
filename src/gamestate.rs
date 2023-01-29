use std::fmt::Debug;

use graphics::Context;
use opengl_graphics::GlGraphics;

use crate::{
    gameobject::{
        entity::registry::{EntityRegistry, RegisteredEntity},
        field::{Field, FieldView},
    },
    rendering::View,
    Camera, ScarabResult,
};

// TODO: rename Gamestate to "Scene"
// Then `Scene`s shouldn't have any concept of "Player inputs" those should all
// be at the application level
#[derive(Debug)]
pub struct Gamestate<E> {
    field: Field,
    field_view: FieldView,
    entity_registry: EntityRegistry<E>,
}

impl<E: RegisteredEntity> Gamestate<E> {
    pub fn new(field: Field, field_view: FieldView) -> Self {
        Self {
            field,
            field_view,
            entity_registry: EntityRegistry::default(),
        }
    }

    pub fn render(&self, camera: &Camera, ctx: Context, gl: &mut GlGraphics) -> ScarabResult<()> {
        self.field_view.render(&self.field, &camera, ctx, gl)?;

        for registered_entity in &self.entity_registry {
            registered_entity.inner_view().render(
                registered_entity.inner_entity(),
                camera,
                ctx,
                gl,
            )?;
        }
        Ok(())
    }

    // TODO: rethink if the entity views should be refs or not
    pub fn register_entity(&mut self, to_register: E) -> ScarabResult<()> {
        self.entity_registry.push(to_register);
        Ok(())
    }

    pub fn get_field(&self) -> &Field {
        &self.field
    }

    pub fn update_entity_inputs(&mut self) -> ScarabResult<()> {
        for registered_entity in &mut self.entity_registry {
            registered_entity.maybe_exhaust_channel()?;
        }
        Ok(())
    }

    pub fn tick_entities(&mut self, dt: f64) -> ScarabResult<()> {
        // fn get_overlaps<'a, E: RegisteredEntity>(
        //     others: OthersIter<'a, E>,
        //     this_entity: &Entity,
        // ) -> Vec<PhysBox> {
        //     others
        //         .filter(|r| r.inner_entity().get_solidity().has_solidity())
        //         .filter_map(|r| {
        //             let projected = r.inner_entity().get_projected_box();
        //             if this_entity.get_box().has_overlap(&projected) {
        //                 Some(projected)
        //             } else if this_entity
        //                 .get_box()
        //                 .has_overlap(r.inner_entity().get_box())
        //             {
        //                 Some(*r.inner_entity().get_box())
        //             } else {
        //                 None
        //             }
        //         })
        //         .collect()
        // }

        for registered_entity in &mut self.entity_registry {
            registered_entity
                .inner_entity_mut()
                .game_tick(&self.field, dt)?;
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
}

// TODO: how to renderrrrr a game state in the different layers?
