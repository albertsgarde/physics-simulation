use serde::{Deserialize, Serialize};

use crate::Vector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    delta_per_tick: f32,
    gravity: Vector,
    width: f32,
    height: f32,
    repulsion_constant: f32,
}

impl Config {
    pub fn new(
        delta_per_tick: f32,
        gravity: Vector,
        width: f32,
        height: f32,
        repulsion_constant: f32,
    ) -> Self {
        Self {
            delta_per_tick,
            gravity,
            width,
            height,
            repulsion_constant,
        }
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
}
