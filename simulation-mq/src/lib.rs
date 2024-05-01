pub mod draw;
mod ui_config;
mod ui_state;

pub use ui_config::UiConfig;
pub use ui_state::UiState;

#[derive(Clone, Copy, Debug)]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32,
}
