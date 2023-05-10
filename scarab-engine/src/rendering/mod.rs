/// All things generic rendering
use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::{error::RenderResult, Camera};

use self::registry::TextureRegistry;

#[cfg(feature = "debug-rendering")]
/// Rendering additional debugging info
pub mod debug;
/// Rendering registries
pub mod registry;
/// Specifically for rendering sprites
pub mod sprite;

/// A trait for types that control how another type is rendered
pub trait View {
    /// The actually rendered type
    type Viewed;

    /// Renders `Viewed` on the screen
    fn render(
        &mut self,
        viewed: &Self::Viewed,
        args: &RenderArgs,
        camera: &Camera,
        ctx: Context,
        texture_registry: &TextureRegistry,
        gl: &mut GlGraphics,
    ) -> RenderResult<()>;
}
