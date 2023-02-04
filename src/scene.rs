use std::fmt::Debug;

use graphics::Context;
use opengl_graphics::GlGraphics;
use serde::{Deserialize, Serialize};

use crate::{
    gameobject::{
        entity::registry::{EntityRegistry, RegisteredEntity},
        field::{Field, FieldView},
        HasSolidity,
    },
    rendering::View,
    Camera, HasBox, HasBoxMut, ScarabResult,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Scene<E> {
    field: Field,
    field_view: FieldView,
    entity_registry: EntityRegistry<E>,
}

impl<E: RegisteredEntity> Scene<E> {
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

    pub fn register_entity(&mut self, to_register: E) -> ScarabResult<()> {
        self.entity_registry.register(to_register)
    }

    pub fn get_field(&self) -> &Field {
        &self.field
    }

    pub fn tick_entities(&mut self, dt: f64) -> ScarabResult<()> {
        for registered_entity in &mut self.entity_registry {
            registered_entity
                .inner_entity_mut()
                .game_tick(&self.field, dt)?;
        }

        // This is kinda gross, but I don't really know how else to do it
        // we'll see later how necessary it is to change
        for this_index in 0..self.entity_registry.len() {
            if let Some(this_one) = self.entity_registry.get_one(this_index) {
                if !this_one.inner_entity().get_solidity().has_solidity() {
                    continue;
                }

                let this_one_box = *this_one.inner_entity().get_box();

                for other_index in 0..this_index {
                    if this_index == other_index {
                        continue;
                    }
                    if let Some(other_one) = self.entity_registry.get_one_mut(other_index) {
                        if other_one.inner_entity().get_solidity().has_solidity() {
                            other_one
                                .inner_entity_mut()
                                .get_box_mut()
                                .shift_to_nonoverlapping(&this_one_box);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // TODO! Find a way to pin the return type of this to a specific type within the registry
    pub fn player_mut(&mut self) -> Option<&mut E> {
        self.entity_registry.iter_mut().next()
    }
}
