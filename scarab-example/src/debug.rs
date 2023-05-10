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
        viewed: &Self::Viewed,
        debug_options: &Self::DebugOptions,
        args: &piston::RenderArgs,
        camera: &scarab_engine::Camera,
        ctx: graphics::Context,
        texture_registry: &scarab_engine::rendering::registry::TextureRegistry,
        gl: &mut opengl_graphics::GlGraphics,
    ) -> scarab_engine::error::RenderResult<()> {
        if debug_options.field_collision_boxes {
            // todo!.
        }

        Ok(())
    }
}
