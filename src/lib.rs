#![feature(get_mut_unchecked)]
#![feature(min_specialization)]
#![feature(trait_alias)]

pub mod app;
pub mod camera;
pub mod error;
pub mod gameobject;
pub mod gamestate;
pub mod rendering;
mod types;

pub use app::App;
pub use camera::Camera;
pub use error::{ScarabError, ScarabResult};
pub use gamestate::Gamestate;
pub use types::*;
