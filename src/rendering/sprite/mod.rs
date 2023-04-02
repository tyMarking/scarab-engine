/// Rendering sprites attached to a game object
use core::fmt::Debug;
use std::{collections::HashMap, hash::Hash, path::PathBuf, time::Instant};

use derivative::Derivative;
use graphics::{Image, ImageSize};
use piston::RenderArgs;
use serde::{Deserialize, Serialize};
use shapes::{Point, Size};

use crate::{gameobject::Entity, Axis, Camera, HasBox, ScarabError, ScarabResult};

use self::sprite_serde::ImageDef;

use super::{registry::TextureRegistry, View};

mod sprite_serde;

#[derive(Derivative, Clone, Serialize, Deserialize)]
#[derivative(Debug)]
/// A view type for displaying a simple static image.
/// Should generally be used wrapped by a [SpriteAnimation].
pub struct SpriteView {
    pos: Point,
    sprite_size: Size,
    #[derivative(Debug = "ignore")]
    #[serde(with = "ImageDef")]
    image: Image,
    texture_path: PathBuf,
}

impl SpriteView {
    /// Creates a new SpriteView.
    /// Displays the Texture at the given path with the given size translated by the given pos
    pub fn new(pos: Point, sprite_size: Size, texture_path: PathBuf) -> ScarabResult<Self> {
        Ok(Self {
            pos,
            sprite_size,
            image: Image::new()
                .rect([-pos.x, -pos.y, sprite_size.w, sprite_size.h])
                .src_rect([0.0, 0.0, sprite_size.w, sprite_size.h]),
            texture_path,
        })
    }

    fn set_src_rect_pos(&mut self, new_pos: Point) {
        if let Some(rect) = self.image.source_rectangle.as_mut() {
            rect[0] = new_pos.x;
            rect[1] = new_pos.y;
        }
    }

    fn render<V: HasBox>(
        &mut self,
        viewed: &V,
        _args: &RenderArgs,
        camera: &Camera,
        ctx: graphics::Context,
        texture_registry: &TextureRegistry,
        gl: &mut opengl_graphics::GlGraphics,
    ) -> ScarabResult<()> {
        if let Some((transform, rect)) = camera.box_renderables(viewed.get_box(), ctx) {
            graphics::rectangle([0.0, 1.0, 1.0, 1.0], rect, transform, gl);
            self.image.draw(
                texture_registry.get_or_default(&self.texture_path),
                &ctx.draw_state,
                transform,
                gl,
            );
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A view type for displaying an animation across a single sprite map
/// Should generally be used wrapped by an [AnimationStateMachine]
pub struct SpriteAnimation {
    /// The spritemap that this animation wraps
    sprite: SpriteView,
    /// The number of frames in the sprite map
    frames_in_sprite_map: usize,
    /// The current frame number in the animation
    frame_num: usize,
    /// The frame rate of the animation in *seconds* per frame
    milliseconds_per_frame: f64,
    /// The axis within the sprite map that adding to gets to the next frame
    animation_direction: Axis,
    /// The timestamp at which the last frame was set
    #[serde(skip)]
    #[serde(default = "Instant::now")]
    last_update: Instant,
}

impl SpriteAnimation {
    /// Creates a new SpriteAnimation using the sprite map at texture_path
    /// `animation_direction`: The axis on the spritemap which adding to gets to the next frame
    /// `frames_in_sprite_map`: Optionally, the number of frames in the animation. If `None` it is calculated from the dimensions of the sprite.
    ///     The method fails if `Some(usize)` and is larger than the sprite along its animation direction
    pub fn new(
        pos: Point,
        sprite_size: Size,
        texture_path: PathBuf,
        milliseconds_per_frame: f64,
        animation_direction: Axis,
        frames_in_sprite_map: Option<usize>,
        registry: &TextureRegistry,
    ) -> ScarabResult<Self> {
        let sprite = SpriteView::new(pos, sprite_size, texture_path)?;

        let map_size = registry.get(&sprite.texture_path).map_or_else(
            || {
                Err(ScarabError::RawString(format!(
                    "could not load texture: {:?}",
                    sprite.texture_path,
                )))
            },
            |texture| Ok(texture.get_size()),
        )?;
        let max_num_frames = match animation_direction {
            Axis::X => (map_size.0 / sprite_size.w as u32) as usize,
            Axis::Y => (map_size.1 / sprite_size.h as u32) as usize,
        };

        let frames_in_sprite_map = if let Some(frames) = frames_in_sprite_map {
            if frames > max_num_frames {
                return Err(ScarabError::RawString(
                    "too many frames for sprite sheet".to_string(),
                ));
            } else {
                frames
            }
        } else {
            max_num_frames
        };

        Ok(Self {
            sprite,
            frames_in_sprite_map,
            frame_num: 0,
            milliseconds_per_frame,
            animation_direction,
            last_update: Instant::now(),
        })
    }

    /// Creates an "Animation" that only displays a single frame
    pub fn new_static_frame(sprite: SpriteView) -> Self {
        Self {
            sprite,
            frames_in_sprite_map: 0,
            frame_num: 0,
            milliseconds_per_frame: 1000.0,
            animation_direction: Axis::X,
            last_update: Instant::now(),
        }
    }

    /// Prepares the animation to be started again.
    fn reset(&mut self) {
        self.frame_num = 0;
        self.last_update = Instant::now()
    }

    fn render<V: HasBox>(
        &mut self,
        viewed: &V,
        args: &RenderArgs,
        camera: &Camera,
        ctx: graphics::Context,
        texture_registry: &TextureRegistry,
        gl: &mut opengl_graphics::GlGraphics,
    ) -> ScarabResult<()> {
        // args.ext_dt is a liar, so we calculate our own dt
        let now = Instant::now();

        let num_new_frames =
            ((now - self.last_update).as_millis() / self.milliseconds_per_frame as u128) as usize;
        if num_new_frames > 0 && self.frames_in_sprite_map > 0 {
            self.last_update = now;
            self.frame_num = (self.frame_num + num_new_frames) % self.frames_in_sprite_map;
            let new_pos = match self.animation_direction {
                Axis::X => [self.frame_num as f64 * self.sprite.sprite_size.w, 0.0].into(),
                Axis::Y => [0.0, self.frame_num as f64 * self.sprite.sprite_size.h].into(),
            };
            self.sprite.set_src_rect_pos(new_pos)
        }

        self.sprite
            .render(viewed, args, camera, ctx, texture_registry, gl)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A viewing type for displaying one of a set of [SpriteAnimation]s
pub struct AnimationStateMachine<S: AnimationStates> {
    current_state: S,
    animations: HashMap<S, SpriteAnimation>,
}

impl<S: AnimationStates> AnimationStateMachine<S> {
    /// The 'animations' must have an entry for 'initial_state'
    pub fn new(initial_state: S, animations: HashMap<S, SpriteAnimation>) -> ScarabResult<Self> {
        let _ = animations
            .get(&initial_state)
            .ok_or_else(|| ScarabError::RawString("missing initial state animation".to_string()));

        Ok(Self {
            current_state: initial_state,
            animations,
        })
    }

    /// Sets the SpriteAnimation for a given state
    pub fn set_state_animation(&mut self, state: S, animation: SpriteAnimation) {
        self.animations.insert(state, animation);
    }

    /// Sets the current state to new_state.
    /// Fails if there is no animation for new_state
    pub fn set_current_state(&mut self, new_state: S) -> ScarabResult<()> {
        if self.animations.contains_key(&new_state) {
            let new_animation = self.animations.get_mut(&new_state).unwrap();
            new_animation.reset();
            self.current_state = new_state;
            Ok(())
        } else {
            Err(ScarabError::RawString(format!(
                "no animation for state: {:?}",
                new_state
            )))
        }
    }
}

impl AnimationStateMachine<StaticAnimation> {
    /// Creates an AnimationStateMachine that always remains on a single animation
    pub fn static_animation(animation: SpriteAnimation) -> Self {
        let mut animations = HashMap::new();
        let current_state = StaticAnimation::default();
        animations.insert(current_state.clone(), animation);
        Self {
            current_state,
            animations,
        }
    }
}

/// Defines the set of states which an [AnimationStateMachine] can render
pub trait AnimationStates: Debug + Eq + Hash
where
    Self: Sized,
{
    /// The viewed type
    type Viewed: HasBox;

    /// Determines the animation state to be rendered based on the status of the viewed type.
    /// if `None` the animation state should not change
    fn next_state(&self, viewed: &Self::Viewed) -> Option<Self>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
/// A set of animation states that always remains on the same state
pub struct StaticAnimation;

impl AnimationStates for StaticAnimation {
    // TODO! make the implementation generic across Viewed types
    type Viewed = Entity;

    fn next_state(&self, _viewed: &Self::Viewed) -> Option<Self> {
        None
    }
}

impl<S: AnimationStates> View for AnimationStateMachine<S> {
    type Viewed = S::Viewed;

    fn render(
        &mut self,
        viewed: &Self::Viewed,
        args: &RenderArgs,
        camera: &Camera,
        ctx: graphics::Context,
        texture_registry: &TextureRegistry,
        gl: &mut opengl_graphics::GlGraphics,
    ) -> ScarabResult<()> {
        self.current_state
            .next_state(viewed)
            .map_or(Ok(()), |s| self.set_current_state(s))
            .unwrap_or_else(|e| {
                println!("Error setting animation state for {:?}: {:?}", self, e);
            });

        let animation = self.animations.get_mut(&self.current_state).unwrap();
        animation.render(viewed, args, camera, ctx, texture_registry, gl)
    }
}
