use serde::{Deserialize, Serialize};

use crate::{Float, Vector};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub seed: u64,
    pub delta_per_tick: Float,
    pub gravity: Vector,
    pub width: Float,
    pub height: Float,
    pub repulsion_constant: Float,
    pub air_resistance: Float,
    pub max_speed: Float,
    pub indentical_particle_repulsion: Float,
    pub repulsion_distance: Float,
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
