mod enemy;
mod player;

pub use enemy::Enemy;
use graphics::{types::Color, Context};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
pub use player::{Player, PlayerAnimations};
use scarab_engine::{
    error::RenderResult,
    gameobject::{
        entity::{
            registry::{RegisteredDebugEntity, RegisteredEntity},
            HasEntity,
        },
        Entity, HasHealth,
    },
    rendering::{
        debug::{DebugView, StandardAndDebugView},
        registry::TextureRegistry,
        sprite::{AnimationStateMachine, StaticAnimation},
        View,
    },
    scene::GameTickArgs,
    Camera, HasBox, HasUuid, ScarabResult,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::debug::DebugOptions;

#[derive(Debug, Serialize, Deserialize)]
pub enum ExampleEntities {
    Player(
        (
            Player,
            StandardAndDebugView<AnimationStateMachine<PlayerAnimations>, EntityDebug>,
        ),
    ),
    Enemy(
        (
            Enemy,
            StandardAndDebugView<AnimationStateMachine<StaticAnimation>, EntityDebug>,
        ),
    ),
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

impl RegisteredDebugEntity for ExampleEntities {
    type DebugOptions = DebugOptions;

    fn render_with_info(
        &mut self,
        debug_options: &Self::DebugOptions,
        args: &RenderArgs,
        camera: &Camera,
        ctx: Context,
        texture_registry: &TextureRegistry,
        gl: &mut GlGraphics,
    ) -> RenderResult<()> {
        match self {
            Self::Player((player, view)) => view.render_with_info(
                &player.get_entity_mut(),
                debug_options,
                args,
                camera,
                ctx,
                texture_registry,
                gl,
            ),
            Self::Enemy((enemy, view)) => view.render_with_info(
                &enemy.get_entity_mut(),
                debug_options,
                args,
                camera,
                ctx,
                texture_registry,
                gl,
            ),
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityDebug {
    pub box_color: Color,
    pub health_color: Color,
}

impl DebugView for EntityDebug {
    type Viewed = Entity;
    type DebugOptions = DebugOptions;

    fn render_with_info(
        &mut self,
        viewed: &Self::Viewed,
        debug_options: &Self::DebugOptions,
        _args: &RenderArgs,
        camera: &Camera,
        ctx: Context,
        _texture_registry: &TextureRegistry,
        gl: &mut GlGraphics,
    ) -> RenderResult<()> {
        if let Some((transform, rect)) = camera.box_renderables(viewed.get_box(), ctx) {
            if debug_options.entity_collision_boxes {
                graphics::rectangle(self.box_color, rect, transform, gl);
            }

            if debug_options.entity_health {
                let border_size = 1.0;
                let height_fraction = 0.3;

                let mut health_rect = rect.clone();
                let max_width = health_rect[2] - 2.0 * border_size;
                let max_height = health_rect[3] - 2.0 * border_size;

                health_rect[2] = viewed.get_health().fraction() * max_width;
                health_rect[3] = height_fraction * max_height;
                health_rect[0] += border_size;
                health_rect[1] += max_height - health_rect[3];

                graphics::rectangle(self.health_color, health_rect, transform, gl);
            }
        }

        Ok(())
    }
}
