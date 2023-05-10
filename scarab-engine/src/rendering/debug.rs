use graphics::Context;
use opengl_graphics::GlGraphics;
use piston::RenderArgs;
use serde::{Deserialize, Serialize};

use crate::{error::RenderResult, Camera};

use super::{registry::TextureRegistry, View};

/// Renders a game object with extra debugging information depending on the given "DebugOptions"
pub trait DebugView {
    /// The gameobject type that this gives debug info for
    type Viewed;
    /// The options which determing which extra info to show
    type DebugOptions;

    /// Renders viewed on the screen with extra information
    fn render_with_info(
        &mut self,
        viewed: &Self::Viewed,
        debug_options: &Self::DebugOptions,
        args: &RenderArgs,
        camera: &Camera,
        ctx: Context,
        texture_registry: &TextureRegistry,
        gl: &mut GlGraphics,
    ) -> RenderResult<()>;
}

/// A convenience struct for combining a separate "standard" and "debug" view into one
#[derive(Debug, Serialize, Deserialize)]
pub struct StandardAndDebugView<V, D> {
    standard_view: V,
    debug_view: D,
}

impl<V, D> From<(V, D)> for StandardAndDebugView<V, D> {
    fn from((standard_view, debug_view): (V, D)) -> Self {
        Self {
            standard_view,
            debug_view,
        }
    }
}

impl<V, D> View for StandardAndDebugView<V, D>
where
    V: View,
    D: DebugView<Viewed = V::Viewed>,
{
    type Viewed = V::Viewed;

    fn render(
        &mut self,
        viewed: &Self::Viewed,
        args: &RenderArgs,
        camera: &Camera,
        ctx: Context,
        texture_registry: &TextureRegistry,
        gl: &mut GlGraphics,
    ) -> RenderResult<()> {
        self.standard_view
            .render(viewed, args, camera, ctx, texture_registry, gl)
    }
}

impl<V, D> DebugView for StandardAndDebugView<V, D>
where
    V: View,
    D: DebugView<Viewed = V::Viewed>,
{
    type Viewed = V::Viewed;
    type DebugOptions = D::DebugOptions;

    fn render_with_info(
        &mut self,
        viewed: &Self::Viewed,
        debug_options: &Self::DebugOptions,
        args: &RenderArgs,
        camera: &Camera,
        ctx: Context,
        texture_registry: &TextureRegistry,
        gl: &mut GlGraphics,
    ) -> RenderResult<()> {
        self.debug_view.render_with_info(
            viewed,
            debug_options,
            args,
            camera,
            ctx,
            texture_registry,
            gl,
        )?;

        self.standard_view
            .render(viewed, args, camera, ctx, texture_registry, gl)
    }
}
