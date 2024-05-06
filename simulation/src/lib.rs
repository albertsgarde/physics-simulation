mod config;
mod event;
mod geometry;
mod particle;
mod state;

pub use config::Config;
pub use event::Event;
pub use geometry::{Location, Vector};
pub use particle::Particle;
pub use state::State;

pub type Float = f64;
pub const PI: Float = std::f64::consts::PI as Float;
