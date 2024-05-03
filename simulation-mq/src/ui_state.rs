use std::iter;

use macroquad::{input, window};
use simulation::{Event, Location, State, Vector};

use crate::{ScreenPosition, UiConfig};

pub struct UiState {
    config: UiConfig,
    screen_width: f32,
    screen_height: f32,
    /// Where in screen coordinates the world origin is drawn.
    offset: Vector,
    scale: f32,
    last_mouse_position: Option<ScreenPosition>,
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
            last_mouse_position: None,
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
        mid_offset + mid_vec
            - 0.5 * self.scale * Vector::new(state.config().width, -state.config().height)
    }

    pub fn world_to_screen(&self, location: Location) -> ScreenPosition {
        ScreenPosition {
            x: location.x * self.scale + self.offset.x,
            y: self.offset.y - (location.y * self.scale),
        }
    }

    pub fn screen_to_world(&self, position: ScreenPosition) -> Location {
        Location {
            x: (position.x - self.offset.x) / self.scale,
            y: (self.screen_height - (position.y - self.offset.y)) / self.scale,
        }
    }

    pub fn set_offset(&mut self, offset: Vector) {
        self.offset = offset;
    }

    pub fn update_window_info(&mut self) {
        self.screen_width = window::screen_width();
        self.screen_height = window::screen_height();
    }

    pub fn handle_input(&mut self) -> impl IntoIterator<Item = Event> + '_ {
        let mouse_position = input::mouse_position();
        let mouse_position = ScreenPosition::from_tuple(mouse_position);
        let mouse_delta = self.last_mouse_position.map(|last| mouse_position - last);

        let mouse_wheel_delta = input::mouse_wheel().1;

        if input::is_mouse_button_down(input::MouseButton::Right) {
            if let Some(mouse_delta) = mouse_delta {
                let mouse_delta = Vector::new(mouse_delta.x, mouse_delta.y);
                self.offset += mouse_delta;
            }
        }

        if mouse_wheel_delta != 0. {
            let scale_factor = (mouse_wheel_delta / 10000. * self.config.zoom_speed()).exp();
            self.scale *= scale_factor;

            let mouse_to_offset = ScreenPosition::from(self.offset) - mouse_position;
            self.offset = Vector::from(mouse_position + mouse_to_offset * scale_factor);
        }

        self.last_mouse_position = Some(mouse_position);

        iter::empty()
    }
}
