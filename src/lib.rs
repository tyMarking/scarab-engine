#![feature(get_mut_unchecked)]
#![feature(min_specialization)]
#![feature(trait_alias)]

mod app;
pub mod camera;
pub mod error;
pub mod gameobject;
pub mod input;
pub mod rendering;
pub mod scene;
mod types;

pub use app::App;
pub use camera::Camera;
pub use error::{PhysicsError, PhysicsResult, ScarabError, ScarabResult};
pub use scene::Scene;
pub use types::*;
