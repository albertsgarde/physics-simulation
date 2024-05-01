use serde::{Deserialize, Serialize};

use crate::particle::Particle;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    AddParticle(Particle),
}
