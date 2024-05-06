use std::{
    iter,
    time::{Duration, Instant},
};

use simulation::{Config, Event, Float, Location, Particle, State, Vector};

pub fn run_bench<F, E>(config: Config, mut events: F, duration: Duration) -> u64
where
    F: FnMut(u64) -> E,
    E: IntoIterator<Item = Event>,
{
    let mut state = State::new(config);
    let start_time = Instant::now();
    let end_time = start_time + duration;
    let mut step_index = 0;
    while Instant::now() < end_time {
        state.tick(events(step_index));
        step_index += 1;
    }
    step_index
}

pub fn run_std_bench(
    config: Config,
    initial_particles: impl AsRef<[Particle]>,
    duration: Duration,
) -> u64 {
    run_bench(
        config,
        |step_index| {
            if step_index == 0 {
                initial_particles.as_ref()
            } else {
                [].as_ref()
            }
            .as_ref()
            .iter()
            .map(|particle| Event::AddParticle(*particle))
        },
        duration,
    )
}

pub fn report_bench_result(bench_name: impl AsRef<str>, step_count: u64, duration: Duration) {
    let name = bench_name.as_ref();
    let steps_per_second = step_count as f32 / duration.as_secs_f32();
    println!("{name}: {steps_per_second} steps/s - Ran {step_count} steps in {duration:?} seconds",);
}

pub fn bench_and_report(
    bench_name: impl AsRef<str>,
    config: &Config,
    initial_particles: impl AsRef<[Particle]>,
    duration: Duration,
) {
    let step_count = run_std_bench(config.clone(), initial_particles, duration);
    report_bench_result(bench_name, step_count, duration);
}

pub fn close_particles(location: Location, velocity: Vector, amount: usize) -> Vec<Particle> {
    iter::repeat(Particle::new(location, velocity))
        .take(amount)
        .collect()
}

pub fn distributed_particles(width: Float, height: Float, amount: usize) -> Vec<Particle> {
    let grid_size = (amount as Float).sqrt().ceil() as usize;
    let particle_x_dist = width / grid_size as Float;
    let particle_y_dist = height / grid_size as Float;
    (0..amount)
        .map(|i| {
            Particle::new(
                Location::new(
                    particle_x_dist * (i % grid_size) as Float + particle_x_dist / 2.,
                    particle_y_dist * (i / grid_size) as Float + particle_y_dist / 2.,
                ),
                Vector::new(0., 0.),
            )
        })
        .collect()
}

pub fn main() {
    let config = Config::default();
    let duration = Duration::from_secs_f32(0.5);

    bench_and_report(
        "warmup",
        &config,
        distributed_particles(64., 64., 100),
        duration,
    );

    bench_and_report("empty_simulation", &config, [].as_ref(), duration);

    for &amount in [1, 10, 100, 1000, 10000].iter() {
        bench_and_report(
            &format!("distributed_particles_{amount}"),
            &config,
            distributed_particles(64., 64., amount),
            duration,
        );
    }
    for &amount in [1, 10, 100, 1000, 10000].iter() {
        bench_and_report(
            &format!("wide_distributed_particles_{amount}"),
            &config,
            distributed_particles(128., 64., amount),
            duration,
        );
    }
}
