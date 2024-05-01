use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    delta_per_tick: f32,
    width: f32,
    height: f32,
}

impl Config {
    pub fn new(delta_per_tick: f32, width: f32, height: f32) -> Self {
        Self {
            delta_per_tick,
            width,
            height,
        }
    }

    pub fn delta_per_tick(&self) -> f32 {
        self.delta_per_tick
    }

    pub fn width(&self) -> f32 {
        self.width
    }

    pub fn height(&self) -> f32 {
        self.height
    }
}
