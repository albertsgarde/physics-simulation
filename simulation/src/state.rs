use serde::{Deserialize, Serialize};

use crate::{config::Config, event::Event, particle::Particle};

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

        self.particles.iter_mut().for_each(|particle| {
            *particle = particle.update_local(self.config.delta_per_tick());
        });

        self.history.push(events);
    }
}
