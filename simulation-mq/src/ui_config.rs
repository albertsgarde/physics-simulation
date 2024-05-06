use crate::{screen_position::ScreenVector, ui_state::UiState};

#[derive(Debug, Clone)]
pub struct UiConfig {
    ticks_per_second: f32,
    initial_offset: ScreenVector,
    initial_scale: f32,
    zoom_speed: f32,
}

impl UiConfig {
    pub fn new(
        ticks_per_second: f32,
        initial_offset: ScreenVector,
        initial_scale: f32,
        zoom_speed: f32,
    ) -> Self {
        Self {
            ticks_per_second,
            initial_offset,
            initial_scale,
            zoom_speed,
        }
    }

    pub fn ticks_per_second(&self) -> f32 {
        self.ticks_per_second
    }

    pub fn initial_offset(&self) -> ScreenVector {
        self.initial_offset
    }

    pub fn initial_scale(&self) -> f32 {
        self.initial_scale
    }

    pub fn zoom_speed(&self) -> f32 {
        self.zoom_speed
    }

    pub fn new_ui_state(&self) -> UiState {
        UiState::new(self.clone())
    }
}
