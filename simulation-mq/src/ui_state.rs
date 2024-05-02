use macroquad::window;
use simulation::{Location, State, Vector};

use crate::{ScreenPosition, UiConfig};

pub struct UiState {
    config: UiConfig,
    screen_width: f32,
    screen_height: f32,
    offset: Vector,
    scale: f32,
}

impl UiState {
    pub fn new(config: UiConfig) -> Self {
        let screen_width = window::screen_width();
        let screen_height = window::screen_height();
        Self {
            screen_width,
            screen_height,
            offset: config.initial_offset(),
            scale: config.initial_scale(),
            config,
        }
    }

    pub fn config(&self) -> &UiConfig {
        &self.config
    }

    pub fn screen_width(&self) -> f32 {
        self.screen_width
    }

    pub fn screen_height(&self) -> f32 {
        self.screen_height
    }

    pub fn offset(&self) -> Vector {
        self.offset
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn offset_from_mid_offset(&self, mid_offset: Vector, state: &State) -> Vector {
        let mid_vec = Vector::new(self.screen_width / 2., self.screen_height / 2.);
        let offset = mid_offset + mid_vec
            - 0.5 * self.scale * Vector::new(state.config().width, state.config().height);
        Vector::new(offset.x, offset.y)
    }

    pub fn world_to_screen(&self, location: Location) -> ScreenPosition {
        ScreenPosition {
            x: location.x * self.scale + self.offset.x,
            y: self.screen_height - (location.y * self.scale + self.offset.y),
        }
    }

    pub fn screen_to_world(&self, position: ScreenPosition) -> Location {
        Location {
            x: (position.x - self.offset.x) / self.scale,
            y: (self.screen_height - position.y - self.offset.y) / self.scale,
        }
    }

    pub fn set_offset(&mut self, offset: Vector) {
        self.offset = offset;
    }

    pub fn update_window_info(&mut self) {
        self.screen_width = window::screen_width();
        self.screen_height = window::screen_height();
    }
}
