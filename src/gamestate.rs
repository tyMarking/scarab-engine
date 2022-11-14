use std::sync::Arc;

use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::UpdateArgs;

use crate::{
    control::{DoUpdate, UpdateChannel},
    gameobject::{entity::Entity, field::Field},
    playercontroller::InputController,
    Camera, ScarabResult, VecNum,
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

    pub fn add_entity(&mut self, mut entity: Entity<N>) {
        entity.set_field(Arc::clone(&self.field));
        self.entities.push(entity);
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
        for i in 0..self.entities.len() {
            self.entities[i].update(dt)?;
        }

        Ok(())
    }

    pub fn input_event(&mut self, input: piston::Input) -> ScarabResult<()> {
        for i in 0..self.input_controllers.len() {
            let _ = self.input_controllers[i].input_event(&input);
        }

        Ok(())
    }
}

// TODO: how to renderrrrr a game state in the different layers?
