use std::sync::Arc;

use graphics::Context;
use opengl_graphics::GlGraphics;

use crate::{
    gameobject::{entity::Entity, field::Field},
    Camera, ScarabResult, VecNum,
};

pub struct Gamestate<N: VecNum> {
    field: Arc<Field>,
    entities: Vec<Entity<N>>,
}

impl<N: VecNum> Gamestate<N> {
    pub fn new(field: Field) -> Self {
        Self {
            field: Arc::new(field),
            entities: vec![],
        }
    }

    pub fn render(&self, camera: &Camera, ctx: Context, gl: &mut GlGraphics) -> ScarabResult<()> {
        self.field.render(&camera, ctx, gl)?;

        camera.render_boxes(&self.entities, ctx, gl)?;

        Ok(())
    }

    pub fn add_entity(&mut self, mut entity: Entity<N>) {
        entity.set_field(Arc::clone(&self.field));
        self.entities.push(entity);
    }

    pub fn player(&self) -> Option<&Entity<N>> {
        self.entities.get(0)
    }

    pub fn player_mut(&mut self) -> Option<&mut Entity<N>> {
        self.entities.get_mut(0)
    }
}

// TODO: how to renderrrrr a game state in the different layers?
