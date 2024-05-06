use std::{
    collections::VecDeque,
    sync::mpsc,
    time::{Duration, Instant},
};

use macroquad::window::{self, Conf};
use simulation::{Config, Event, Location, Particle, State, Vector};
use simulation_mq::{draw, UiConfig};

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

    let ui_config: UiConfig = UiConfig::new(4000., Vector::new(0., 0.), 10., 3.);

    let mut ui_state = ui_config.new_ui_state();
    ui_state.set_offset(ui_state.offset_from_mid_offset(Vector::new(0., 0.), &state));

    let (event_sender, event_receiver) = mpsc::channel();
    let (state_sender, mut state_receiver) = watch::channel(state.clone());
    let (tps_sender, mut tps_receiver) = watch::channel(0.);

    let _ = std::thread::spawn(move || {
        let mut next_iteration = Instant::now();
        let mut iteration_times = VecDeque::new();
        loop {
            iteration_times.push_back(Instant::now());
            while let Some(true) = iteration_times
                .front()
                .map(|t| t.elapsed() > Duration::from_secs(1))
            {
                iteration_times.pop_front();
            }
            tps_sender.send(iteration_times.len() as f32);

            next_iteration += Duration::from_secs(1).div_f32(ui_config.ticks_per_second());

            state.tick(event_receiver.try_iter());
            state_sender.send(state.clone());

            if next_iteration > Instant::now() {
                std::thread::sleep(next_iteration - Instant::now());
            } else {
                next_iteration = Instant::now();
            }
        }
    });

    for _ in 0..1000 {
        event_sender
            .send(Event::AddParticle(Particle::new(
                Location::new(32., 32.),
                Vector::new(0., 1.),
            )))
            .unwrap();
    }

    let mut state = state_receiver
        .get_if_new()
        .expect("State should not have been seen yet.");

    let mut tps = 0.;

    loop {
        for event in ui_state.handle_input() {
            event_sender.send(event).unwrap();
        }

        ui_state.update_window_info();

        state = state_receiver.get_if_new().unwrap_or(state);
        tps = tps_receiver.get() * 0.2 + tps * 0.8;

        draw::draw(&state, &ui_state, tps as u32);

        window::next_frame().await
    }
}
