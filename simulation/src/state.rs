use serde::{Deserialize, Serialize};

use crate::{config::Config, event::Event, particle::Particle, Vector};

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

        let particle_forces = self
            .particles
            .iter()
            .enumerate()
            .map(|(index, &particle)| {
                self.config.gravity()
                    + self
                        .particles
                        .iter()
                        .enumerate()
                        .filter_map(|(other_index, &other_particle)| {
                            if index != other_index {
                                Some(particle.repulsion_from(&other_particle))
                            } else {
                                None
                            }
                        })
                        .sum::<Vector>()
                        * self.config.repulsion_constant()
            })
            .collect::<Vec<_>>();

        self.particles
            .iter_mut()
            .zip(particle_forces.iter())
            .for_each(|(particle, &force)| {
                particle.velocity += force * self.config.delta_per_tick();
                particle.location += particle.velocity * self.config.delta_per_tick();

                particle.rebound((0., self.config.width()), (0., self.config.height()));
            });

        self.history.push(events);
    }
}
