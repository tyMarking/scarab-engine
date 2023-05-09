mod enemy;
mod player;

pub use enemy::Enemy;
use piston::RenderArgs;
pub use player::{Player, PlayerAnimations};
use scarab_engine::{
    error::RenderResult,
    gameobject::{entity::registry::RegisteredEntity, Entity},
    rendering::{
        registry::TextureRegistry,
        sprite::{AnimationStateMachine, StaticAnimation},
        View,
    },
    HasUuid,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum ExampleEntities {
    Player((Player, AnimationStateMachine<PlayerAnimations>)),
    Enemy((Enemy, AnimationStateMachine<StaticAnimation>)),
}

impl RegisteredEntity for ExampleEntities {
    fn inner_entity(&self) -> &Entity {
        match self {
            Self::Player((player, _view)) => &player.entity,
            Self::Enemy((enemy, _view)) => &enemy.entity,
        }
    }

    fn inner_entity_mut(&mut self) -> &mut Entity {
        match self {
            Self::Player((player, _view)) => &mut player.entity,
            Self::Enemy((enemy, _view)) => &mut enemy.entity,
        }
    }

    fn render(
        &mut self,
        args: &RenderArgs,
        camera: &scarab_engine::Camera,
        ctx: graphics::Context,
        texture_registry: &TextureRegistry,
        gl: &mut opengl_graphics::GlGraphics,
    ) -> RenderResult<()> {
        match self {
            Self::Player((player, view)) => {
                view.render(&player.entity, args, camera, ctx, texture_registry, gl)
            }
            Self::Enemy((enemy, view)) => {
                view.render(&enemy.entity, args, camera, ctx, texture_registry, gl)
            }
        }
    }
}

impl HasUuid for ExampleEntities {
    fn uuid(&self) -> Uuid {
        match self {
            ExampleEntities::Player((player, _view)) => player.entity.uuid(),
            ExampleEntities::Enemy((enemy, _view)) => enemy.entity.uuid(),
        }
    }
}
