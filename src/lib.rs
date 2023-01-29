#![feature(get_mut_unchecked)]
#![feature(min_specialization)]
#![feature(trait_alias)]

pub mod camera;
pub mod error;
pub mod gameobject;
pub mod scene;
pub mod input;
pub mod rendering;
mod types;
pub mod utils;

pub use camera::Camera;
pub use error::{ScarabError, ScarabResult};
pub use scene::Scene;
pub use types::*;
