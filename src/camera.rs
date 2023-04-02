use graphics::{types::Scalar, Context, Transformed};
use serde::{Deserialize, Serialize};
use shapes::Point;

use crate::PhysBox;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Controls how the pixel art is rendered and maintained through play
pub struct Camera {
    /// The number of screen pixels per tile
    pixels_per_tile: u32,
    /// The camera's position and dimensions in world coordinates
    physbox: PhysBox,
    /// The pixel resoultion of the camera
    resolution: [u32; 2],
}

impl Camera {
    /// Makes a new camera
    pub fn new(pixels_per_tile: u32, physbox: PhysBox, resolution: [u32; 2]) -> Self {
        Self {
            pixels_per_tile,
            physbox,
            resolution,
        }
    }

    /// Creates a trasnform matrix for the given point from world coordinates to screen coordinates
    pub fn transform(&self, ctx: &Context, pos: Point) -> [[f64; 3]; 2] {
        let top_left = pos - *self.physbox.pos();
        let top_left_scaled = top_left * self.pixels_per_tile.into();
        // let t_pos = (pos - self.physbox.pos().convert_n()) * self.pixels_per_tile.into();

        ctx.transform.trans(top_left_scaled.x, top_left_scaled.y)
    }

    /// The actual screen pixels per pixel-art-pixel for the camera
    pub fn pixels_per_tile(&self) -> u32 {
        self.pixels_per_tile
    }

    /// Gives the simple transform and redering rectangle for a 2D PhysBox
    pub fn box_renderables(
        &self,
        physbox: &PhysBox,
        ctx: Context,
    ) -> Option<([[f64; 3]; 2], [f64; 4])> {
        if physbox.has_overlap(&self.physbox) {
            let transform = self.transform(&ctx, *physbox.pos());
            let [x1, y1]: [Scalar; 2] = (*physbox.size()).into();
            let rect = graphics::rectangle::rectangle_by_corners(
                0.0,
                0.0,
                x1 * self.pixels_per_tile as f64,
                y1 * self.pixels_per_tile as f64,
            );

            Some((transform, rect))
        } else {
            None
        }
    }
}
