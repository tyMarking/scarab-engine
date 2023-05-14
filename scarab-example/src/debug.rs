use scarab_engine::{
    gameobject::field::Field,
    rendering::{debug::DebugView, Camera},
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugOptions {
    pub entity_collision_boxes: bool,
    pub entity_health: bool,
    pub field_collision_boxes: bool,
    pub attack_cooldowns: bool,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FieldDebug {}

impl<'a> DebugView for FieldDebug {
    type DebugOptions = DebugOptions;
    type Viewed = Field;

    fn render_with_info(
        &mut self,
        _viewed: &Self::Viewed,
        _debug_options: &Self::DebugOptions,
        _args: &piston::RenderArgs,
        _camera: &Camera,
        _ctx: graphics::Context,
        _texture_registry: &scarab_engine::rendering::registry::TextureRegistry,
        _gl: &mut opengl_graphics::GlGraphics,
    ) -> scarab_engine::error::RenderResult<()> {
        // if debug_options.field_collision_boxes {
        // TODO: this is fully redundant since I'm using the [FieldColorView], so I'm chosing not
        // to implement it at the moment
        // }

        Ok(())
    }
}
