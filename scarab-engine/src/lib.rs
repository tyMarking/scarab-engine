#![warn(missing_docs)]

//! The Scarab Engine Library
///
/// more documentation coming soon

/// The trait for running an app at a high level
mod app;
/// The camera controlling how rendering happens
pub mod camera;
/// Common error and result types
pub mod error;
/// Game objects
pub mod gameobject;
/// Player input
pub mod input;
/// Rendering everything
pub mod rendering;
/// The scene wrapping game objects
pub mod scene;
/// Generic types
mod types;

pub use app::App;
pub use camera::Camera;
pub use error::{PhysicsError, PhysicsResult, ScarabError, ScarabResult};
pub use glutin_window::GlutinWindow;
pub use scene::Scene;
pub use types::*;
pub use winit::dpi::LogicalSize;
