use macroquad::window;
use simulation::{Location, Particle, State};

use crate::UiConfig;

pub fn state_to_screen(location: Location, ui_config: &UiConfig) -> Location {
    let screen_x = location.x();
    let screen_y = window::screen_height() - location.y();

    Location::new(screen_x, screen_y)
}

pub fn draw_particle(particle: Particle, ui_config: &UiConfig) {
    let screen_location = state_to_screen(particle.location(), ui_config);

    macroquad::shapes::draw_circle(
        screen_location.x(),
        screen_location.y(),
        5.,
        macroquad::color::WHITE,
    );
}

pub fn draw(state: &State, ui_config: &UiConfig) {
    let particles = state.particles();

    particles
        .iter()
        .for_each(|&particle| draw_particle(particle, ui_config));
}
