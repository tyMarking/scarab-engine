use crate::Color;

pub trait Renderable {
    // fn render(&self, camera: Camera, ctx: Context, gl: &mut GlGraphics) -> ScarabResult<()>;

    fn color(&self) -> &Color;
}
