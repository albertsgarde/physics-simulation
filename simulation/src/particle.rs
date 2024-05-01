use serde::{Deserialize, Serialize};

use crate::geometry::{Location, Vector};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Particle {
    location: Location,
    velocity: Vector,
}

impl Particle {
    pub fn new(location: Location, velocity: Vector) -> Self {
        Particle { location, velocity }
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn velocity(&self) -> Vector {
        self.velocity
    }

    pub fn add_velocity(self, velocity: Vector) -> Self {
        Particle {
            location: self.location,
            velocity: self.velocity + velocity,
        }
    }

    pub fn update_local(self, delta: f32) -> Self {
        Particle {
            location: self.location + self.velocity * delta,
            velocity: self.velocity,
        }
    }
}
