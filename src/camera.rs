use graphics::{Context, Transformed};
use opengl_graphics::GlGraphics;

use crate::{rendering::Renderable, HasBox, PhysBox, ScarabResult, TileVec, VecNum};

#[derive(Debug, Clone)]
pub struct Camera {
    /// The number of screen pixels per tile
    pixels_per_tile: u32,
    /// The camera's position and dimensions by tiles
    physbox: PhysBox<u32>,
    /// The pixel resoultion of the camera
    resolution: TileVec<u32>,
}

impl Camera {
    pub fn new(pixels_per_tile: u32, physbox: PhysBox<u32>, resolution: TileVec<u32>) -> Self {
        Self {
            pixels_per_tile,
            physbox,
            resolution,
        }
    }

    pub fn transform<N: VecNum>(&self, ctx: &Context, pos: TileVec<N>) -> [[f64; 3]; 2] {
        let t_pos = (pos - self.physbox.pos().convert_n()) * self.pixels_per_tile.into();

        ctx.transform.trans(t_pos.x().into(), t_pos.y().into())
    }

    pub fn pixels_per_tile(&self) -> u32 {
        self.pixels_per_tile
    }

    pub fn render_boxes<N, B>(
        &self,
        boxes: &[B],
        ctx: Context,
        gl: &mut GlGraphics,
    ) -> ScarabResult<()>
    where
        N: VecNum,
        B: HasBox<N> + Renderable,
    {
        for b in boxes {
            let physbox = b.get_box();
            if physbox.has_overlap(&self.physbox.convert_n()) {
                let transform = self.transform(&ctx, physbox.pos());
                let (x1, y1): (f64, f64) = physbox.size().convert_n().into();
                let rect = graphics::rectangle::rectangle_by_corners(
                    0.0,
                    0.0,
                    x1 * self.pixels_per_tile as f64,
                    y1 * self.pixels_per_tile as f64,
                );

                graphics::rectangle(b.color().to_owned(), rect, transform, gl);
            }
        }
        Ok(())
    }
}
