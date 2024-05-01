use serde::{Deserialize, Serialize};

use crate::Vector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    delta_per_tick: f32,
    gravity: Vector,
    width: f32,
    height: f32,
    repulsion_constant: f32,
    air_resistance: f32,
    max_speed: f32,
}

impl Config {
    pub fn new(
        delta_per_tick: f32,
        gravity: Vector,
        width: f32,
        height: f32,
        repulsion_constant: f32,
        air_resistance: f32,
        max_speed: f32,
    ) -> Self {
        Self {
            delta_per_tick,
            gravity,
            width,
            height,
            repulsion_constant,
            air_resistance,
            max_speed,
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

    pub fn air_resistance(&self) -> f32 {
        self.air_resistance
    }

    pub fn max_speed(&self) -> f32 {
        self.max_speed
    }
}
