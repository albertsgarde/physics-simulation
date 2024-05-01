use macroquad::{
    input, time,
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
    let ui_config = UiConfig::new(20.);

    let config = Config::new(0.05, 900., 900.);

    let mut state = State::new(config);

    let mut tick = Tick::new();

    let mut events = vec![Event::AddParticle(Particle::new(
        Location::new(450., 450.),
        Vector::new(0., 0.),
    ))];

    loop {
        tick.elapse_time(
            time::get_frame_time() * ui_config.ticks_per_second(),
            || state.tick(events.drain(..)),
        );

        draw::draw(&state, &ui_config);

        window::next_frame().await
    }
}
