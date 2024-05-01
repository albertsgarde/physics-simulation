use simulation::{Particle, State};

use crate::UiConfig;

pub fn draw_particle(particle: Particle, ui_config: &UiConfig) {
    let location = particle.location();
    let velocity = particle.velocity();

    let velocity = velocity.normalize();

    macroquad::shapes::draw_circle(location.x(), location.y(), 5., macroquad::color::WHITE);
}

pub fn draw(state: &State, ui_config: &UiConfig) {
    let particles = state.particles();

    particles
        .iter()
        .for_each(|&particle| draw_particle(particle, ui_config));
}
