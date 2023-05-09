use std::fmt::Debug;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use serde::{Deserialize, Serialize};

use crate::{
    gameobject::{
        entity::registry::{EntityRegistry, RegisteredEntity},
        field::Field,
        HasSolidity,
    },
    rendering::{registry::TextureRegistry, View},
    Camera, HasBox, HasBoxMut, PhysicsResult, ScarabResult,
};

#[derive(Debug, Serialize, Deserialize)]
/// A wrapper over all things in the app right now
pub struct Scene<E, V> {
    field: Field,
    field_view: V,
    entity_registry: EntityRegistry<E>,
}

impl<E: RegisteredEntity, V: View<Viewed = Field>> Scene<E, V> {
    /// Initializes a new scene with the given field, field view and no entities
    pub fn new(field: Field, field_view: V) -> Self {
        Self {
            field,
            field_view,
            entity_registry: EntityRegistry::default(),
        }
    }

    /// Renders everything in the scene
    pub fn render(
        &mut self,
        args: &RenderArgs,
        camera: &Camera,
        ctx: Context,
        texture_registry: &TextureRegistry,
        gl: &mut GlGraphics,
    ) -> ScarabResult<()> {
        self.field_view
            .render(&mut self.field, args, &camera, ctx, texture_registry, gl)?;

        for registered_entity in &mut self.entity_registry {
            registered_entity.render(args, camera, ctx, texture_registry, gl)?;
        }
        Ok(())
    }

    /// Registers a new entity to te scene
    pub fn register_entity(&mut self, to_register: E) -> PhysicsResult<()> {
        self.entity_registry.register(to_register)
    }

    /// Gets a reference to the scene's [Field]
    pub fn get_field(&self) -> &Field {
        &self.field
    }

    /// Runs the physics update for all of the scene's entities
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
    /// Optionally returns a mutable reference to the scene's player
    pub fn player_mut(&mut self) -> Option<&mut E> {
        self.entity_registry.iter_mut().next()
    }
}
