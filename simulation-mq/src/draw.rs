use macroquad::{
    color::{self, Color},
    shapes,
};
use simulation::{Location, Particle, State};

use crate::{ui_state::UiState, ScreenPosition};

pub fn draw_circle(position: ScreenPosition, radius: f32, color: Color) {
    shapes::draw_circle(position.x, position.y, radius, color);
}

pub fn draw_particle(particle: Particle, ui_state: &UiState) {
    let screen_position = ui_state.world_to_screen(particle.location());

    draw_circle(screen_position, 1., color::WHITE);
}

pub fn draw(state: &State, ui_state: &UiState) {
    let lower_left = ui_state.world_to_screen(Location::new(0., 0.));
    let upper_right =
        ui_state.world_to_screen(Location::new(state.config().width, state.config().height));

    macroquad::shapes::draw_rectangle(
        lower_left.x,
        upper_right.y,
        upper_right.x - lower_left.x,
        lower_left.y - upper_right.y,
        color::DARKGRAY,
    );

    let particles = state.particles();

    particles
        .iter()
        .for_each(|&particle| draw_particle(particle, ui_state));
}
