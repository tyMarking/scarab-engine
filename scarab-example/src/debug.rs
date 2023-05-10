use scarab_engine::{gameobject::Field, rendering::debug::DebugView};
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

impl DebugView for FieldDebug {
    type DebugOptions = DebugOptions;
    type Viewed = Field;

    fn render_with_info(
        &mut self,
        _viewed: &Self::Viewed,
        _debug_options: &Self::DebugOptions,
        _args: &piston::RenderArgs,
        _camera: &scarab_engine::Camera,
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
