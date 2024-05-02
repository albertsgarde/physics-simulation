use rand::SeedableRng;
use rand_xoshiro::Xoshiro256PlusPlus;
use serde::{Deserialize, Serialize};

use crate::{config::Config, event::Event, particle::Particle, Vector};

type Rng = Xoshiro256PlusPlus;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    history: Vec<Vec<Event>>,
    config: Config,
    particles: Vec<Particle>,
    rng: Rng,
}

impl State {
    pub fn new(config: Config) -> Self {
        State {
            history: Vec::new(),
            particles: Vec::new(),
            rng: Rng::seed_from_u64(config.seed),
            config,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    fn apply_particle_repulsion(
        particle_a: &mut Particle,
        particle_b: &mut Particle,
        rng: &mut Rng,
        config: &Config,
    ) {
        if particle_a.location.dist_squared(particle_b.location) > config.repulsion_distance.powi(2)
        {
            return;
        }
        let force = if let Some(repulsion) = particle_a.repulsion_from(particle_b) {
            config.repulsion_constant * repulsion
        } else {
            Vector::random_unit(rng) * config.indentical_particle_repulsion
        };
        let velocity_delta = force * config.delta_per_tick;
        particle_a.velocity += velocity_delta;
        particle_b.velocity -= velocity_delta;
    }

    pub fn tick(&mut self, events: impl IntoIterator<Item = Event>) {
        let events = events.into_iter().collect::<Vec<_>>();

        events.iter().for_each(|event| match event {
            Event::AddParticle(particle) => {
                self.particles.push(*particle);
            }
        });

        self.particles.sort_unstable_by(|&a, &b| {
            a.location
                .x
                .partial_cmp(&b.location.x)
                .expect("Assume that locations are all finite (and therefore non-NaN).")
        });

        for index in 0..self.particles.len() {
            let (particle_a, right) = self.particles[index..]
                .split_first_mut()
                .expect("Index is valid.");
            let particle_a_x = particle_a.location.x;
            for particle_b in right.iter_mut().take_while(|particle_b| {
                particle_b.location.x - particle_a_x <= self.config.repulsion_distance
            }) {
                Self::apply_particle_repulsion(particle_a, particle_b, &mut self.rng, &self.config);
            }
        }

        self.particles.iter_mut().for_each(|particle| {
            particle.velocity += self.config.delta_per_tick
                * (self.config.gravity + particle.air_resistance() * self.config.air_resistance);

            if particle.velocity.norm_squared() > self.config.max_speed * self.config.max_speed {
                particle.velocity = particle.velocity.normalize() * self.config.max_speed;
            }

            particle.location += particle.velocity * self.config.delta_per_tick;

            particle.rebound((0., self.config.width), (0., self.config.height));
        });

        self.history.push(events);
    }
}
