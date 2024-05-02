use serde::{Deserialize, Serialize};

use crate::Vector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub seed: u64,
    pub delta_per_tick: f32,
    pub gravity: Vector,
    pub width: f32,
    pub height: f32,
    pub repulsion_constant: f32,
    pub air_resistance: f32,
    pub max_speed: f32,
    pub indentical_particle_repulsion: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            seed: 0,
            delta_per_tick: 0.01,
            gravity: Vector::new(0., -9.81),
            width: 64.,
            height: 64.,
            repulsion_constant: 5.,
            air_resistance: 0.1,
            max_speed: 100.,
            indentical_particle_repulsion: 1e-2,
        }
    }
}

impl Config {
    pub fn seed(&self) -> u64 {
        self.seed
    }

    pub fn delta_per_tick(&self) -> f32 {
        self.delta_per_tick
    }

    pub fn gravity(&self) -> Vector {
        self.gravity
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }

    pub fn repulsion_constant(&self) -> f32 {
        self.repulsion_constant
    }

    pub fn air_resistance(&self) -> f32 {
        self.air_resistance
    }

    pub fn max_speed(&self) -> f32 {
        self.max_speed
    }

    pub fn indentical_particle_repulsion(&self) -> f32 {
        self.indentical_particle_repulsion
    }
}
