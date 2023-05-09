mod enemy;
mod player;

pub use enemy::Enemy;
use piston::RenderArgs;
pub use player::{Player, PlayerAnimations};
use scarab_engine::{
    error::RenderResult,
    gameobject::{
        entity::{registry::RegisteredEntity, HasEntity},
        Entity,
    },
    rendering::{
        registry::TextureRegistry,
        sprite::{AnimationStateMachine, StaticAnimation},
        View,
    },
    scene::GameTickArgs,
    HasUuid, ScarabResult,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub enum ExampleEntities {
    Player((Player, AnimationStateMachine<PlayerAnimations>)),
    Enemy((Enemy, AnimationStateMachine<StaticAnimation>)),
}

impl RegisteredEntity for ExampleEntities {
    type Player<'e, 's: 'e> = Player;

    fn inner_entity(&self) -> &Entity {
        match self {
            Self::Player((player, _view)) => &player.get_entity(),
            Self::Enemy((enemy, _view)) => &enemy.get_entity(),
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
            Self::Player((player, view)) => view.render(
                &player.get_entity_mut(),
                args,
                camera,
                ctx,
                texture_registry,
                gl,
            ),
            Self::Enemy((enemy, view)) => view.render(
                &enemy.get_entity_mut(),
                args,
                camera,
                ctx,
                texture_registry,
                gl,
            ),
        }
    }

    fn maybe_player<'e, 's: 'e>(
        &self,
    ) -> Option<&<ExampleEntities as RegisteredEntity>::Player<'e, 's>> {
        match self {
            Self::Player((p, _)) => Some(p),
            _ => None,
        }
    }

    fn maybe_player_mut<'e, 's: 'e>(
        &mut self,
    ) -> Option<&mut <ExampleEntities as RegisteredEntity>::Player<'e, 's>> {
        match self {
            Self::Player((p, _)) => Some(p),
            _ => None,
        }
    }

    fn game_tick(&mut self, this_idx: usize, args: &mut GameTickArgs<Self>) -> ScarabResult<()> {
        match self {
            ExampleEntities::Player((player, _)) => player.game_tick(this_idx, args),
            ExampleEntities::Enemy((enemy, _)) => {
                enemy.entity.game_tick(args).map_err(|e| e.into())
            }
        }
    }
}

impl HasUuid for ExampleEntities {
    fn uuid(&self) -> Uuid {
        match self {
            ExampleEntities::Player((player, _view)) => player.uuid(),
            ExampleEntities::Enemy((enemy, _view)) => enemy.uuid(),
        }
    }
}
