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
    pub repulsion_distance: f32,
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
            repulsion_distance: 10.,
        }
    }
}
