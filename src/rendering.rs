use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;

use crate::{Camera, ScarabResult};

pub trait View {
    type Viewed;

    fn render(
        &mut self,
        viewed: &Self::Viewed,
        args: &RenderArgs,
        camera: &Camera,
        ctx: Context,
        gl: &mut GlGraphics,
    ) -> ScarabResult<()>;
}
