use std::iter;

use macroquad::{
    time,
    window::{self, Conf},
};
use simulation::{Config, Event, Location, Particle, State, Vector};
use simulation_mq::{draw, UiConfig};

trait TickFunction: FnMut() {}

impl<F> TickFunction for F where F: FnMut() {}

struct Tick {
    cur_delta: f32,
}

impl Tick {
    pub fn new() -> Self {
        Self { cur_delta: 0. }
    }

    pub fn elapse_time<F: TickFunction>(&mut self, delta: f32, mut tick_function: F) {
        self.cur_delta += delta;
        while self.cur_delta >= 1. {
            tick_function();
            self.cur_delta -= 1.;
        }
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Simulation".to_owned(),
        window_width: 900,
        window_height: 900,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let config = Config::default();

    let mut state = State::new(config);

    let ui_config: UiConfig = UiConfig::new(400., Vector::new(0., 0.), 10., 3.);

    let mut ui_state = ui_config.new_ui_state();
    ui_state.set_offset(ui_state.offset_from_mid_offset(Vector::new(0., 0.), &state));

    let mut tick = Tick::new();

    let mut events: Vec<_> = iter::repeat(Event::AddParticle(Particle::new(
        Location::new(32., 32.),
        Vector::new(0., 1.),
    )))
    .take(1000)
    .collect();

    loop {
        events.extend(ui_state.handle_input());

        tick.elapse_time(
            time::get_frame_time() * ui_config.ticks_per_second(),
            || state.tick(events.drain(..)),
        );

        ui_state.update_window_info();

        draw::draw(&state, &ui_state);

        window::next_frame().await
    }
}
