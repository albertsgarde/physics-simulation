use simulation::Vector;

use crate::ui_state::UiState;

#[derive(Debug, Clone)]
pub struct UiConfig {
    ticks_per_second: f32,
    initial_offset: Vector,
    initial_scale: f32,
}

impl UiConfig {
    pub fn new(ticks_per_second: f32, initial_offset: Vector, initial_scale: f32) -> Self {
        Self {
            ticks_per_second,
            initial_offset,
            initial_scale,
        }
    }

    pub fn ticks_per_second(&self) -> f32 {
        self.ticks_per_second
    }

    pub fn initial_offset(&self) -> Vector {
        self.initial_offset
    }

    pub fn initial_scale(&self) -> f32 {
        self.initial_scale
    }

    pub fn new_ui_state(&self) -> UiState {
        UiState::new(self.clone())
    }
}
