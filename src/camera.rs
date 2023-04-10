use graphics::{
    types::{Color, Scalar},
    Context, Transformed,
};
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use serde::{Deserialize, Serialize};
use shapes::Point;

use crate::PhysBox;

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Controls how the pixel art is rendered and maintained through play
///
/// A point is usually what is thought of as a pixel on the monitor, but this is
/// not always true. See [Size] for more information.
///
/// A pixel is one single square in the pixel art graphics, and is also the unit size for physics operations
pub struct Camera {
    /// The number of "points" per "pixel", see struct level documentation for more
    points_per_pixel: f64,
    /// The camera's position and dimensions in world coordinates
    physbox: PhysBox,
    /// The window's size in points
    window_size: [f64; 2],
    /// The width of each vertical bar necessary to fill up the window (in points)
    vertical_bar_width: f64,
    /// The width of each horizontal bar necessary to fill up the window (in points)
    horizontal_bar_height: f64,
}

impl Camera {
    /// Makes a new camera
    pub fn new<S>(physbox: PhysBox, window_size: S) -> Self
    where
        S: From<[f64; 2]>,
        [f64; 2]: From<S>,
    {
        let mut s = Self {
            points_per_pixel: 1.0,
            physbox,
            window_size: window_size.into(),
            vertical_bar_width: 0.0,
            horizontal_bar_height: 0.0,
        };

        s.set_window_size::<[f64; 2]>(s.window_size);
        s
    }

    /// Handles when the window size is changed for updating the points per pixel
    /// May return a preferred new window size if the camera prefers it
    pub fn set_window_size<S>(&mut self, new_window_size: S) -> Option<S>
    where
        S: From<[f64; 2]>,
        [f64; 2]: From<S>,
    {
        let mut override_new_window = false;
        let [mut w_w, mut h_w]: [f64; 2] = new_window_size.into();
        if w_w < self.physbox.size().w {
            override_new_window = true;
            w_w = self.physbox.size().w;
        }

        if h_w < self.physbox.size().h {
            override_new_window = true;
            h_w = self.physbox.size().h;
        }

        self.window_size = [w_w, h_w];
        self.points_per_pixel = f64::min(w_w / self.physbox.size().w, h_w / self.physbox.size().h);

        self.vertical_bar_width = (w_w - self.physbox.size().w * self.points_per_pixel) / 2.0;
        self.horizontal_bar_height = (h_w - self.physbox.size().h * self.points_per_pixel) / 2.0;

        if override_new_window {
            Some([w_w, h_w].into())
        } else {
            None
        }
    }

    /// Creates a trasnform matrix for the given point from world coordinates to screen coordinates
    pub fn transform(&self, ctx: &Context, pos: Point) -> [[f64; 3]; 2] {
        let top_left = pos - *self.physbox.pos();
        let top_left_scaled = top_left * self.points_per_pixel.into()
            + [self.vertical_bar_width, self.horizontal_bar_height];

        ctx.transform.trans(top_left_scaled.x, top_left_scaled.y)
    }

    /// The actual screen "points" per pixel-art-pixel for the camera
    pub fn points_per_pixel(&self) -> f64 {
        self.points_per_pixel
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
                x1 * self.points_per_pixel as f64,
                y1 * self.points_per_pixel as f64,
            );

            Some((transform, rect))
        } else {
            None
        }
    }

    /// Renders the (usually black) bars along the edges of the screen so that only
    /// the camera's allotted pixels are rendered
    pub fn render_gutters(
        &self,
        color: Color,
        _args: &RenderArgs,
        ctx: Context,
        gl: &mut GlGraphics,
    ) {
        graphics::rectangle(
            color,
            [0.0, 0.0, self.vertical_bar_width, self.window_size[1]],
            ctx.transform,
            gl,
        );

        graphics::rectangle(
            color,
            [
                self.window_size[0] - self.vertical_bar_width,
                0.0,
                self.vertical_bar_width,
                self.window_size[1],
            ],
            ctx.transform,
            gl,
        );

        graphics::rectangle(
            color,
            [0.0, 0.0, self.window_size[0], self.horizontal_bar_height],
            ctx.transform,
            gl,
        );

        graphics::rectangle(
            color,
            [
                0.0,
                self.window_size[1] - self.horizontal_bar_height,
                self.window_size[0],
                self.horizontal_bar_height,
            ],
            ctx.transform,
            gl,
        );
    }
}
