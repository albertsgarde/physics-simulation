use serde::{Deserialize, Serialize};

use crate::{config::Config, event::Event, particle::Particle, utils, Vector};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct State {
    history: Vec<Vec<Event>>,
    config: Config,
    particles: Vec<Particle>,
}

impl State {
    pub fn new(config: Config) -> Self {
        State {
            history: Vec::new(),
            config,
            particles: Vec::new(),
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn particles(&self) -> &[Particle] {
        &self.particles
    }

    pub fn tick(&mut self, events: impl IntoIterator<Item = Event>) {
        let events = events.into_iter().collect::<Vec<_>>();

        events.iter().for_each(|event| match event {
            Event::AddParticle(particle) => {
                self.particles.push(*particle);
            }
        });

        utils::all_pairs_mut(self.particles.as_mut_slice(), |particle_a, particle_b| {
            let force = if let Some(repulsion) = particle_a.repulsion_from(particle_b) {
                self.config.repulsion_constant() * repulsion
            } else {
                Vector::new(1e-3, 0.)
            };
            let velocity_delta = force * self.config.delta_per_tick();
            particle_a.velocity += velocity_delta;
            particle_b.velocity -= velocity_delta;
        });

        self.particles.iter_mut().for_each(|particle| {
            particle.velocity += self.config.delta_per_tick()
                * (self.config.gravity()
                    + particle.air_resistance() * self.config.air_resistance());

            if particle.velocity.norm_squared() > self.config.max_speed() * self.config.max_speed()
            {
                particle.velocity = particle.velocity.normalize() * self.config.max_speed();
            }

            particle.location += particle.velocity * self.config.delta_per_tick();

            particle.rebound((0., self.config.width()), (0., self.config.height()));
        });

        self.history.push(events);
    }
}
