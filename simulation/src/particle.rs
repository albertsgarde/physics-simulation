use serde::{Deserialize, Serialize};

use crate::{
    geometry::{Location, Vector},
    Float,
};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Particle {
    pub location: Location,
    pub velocity: Vector,
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

    pub fn repulsion_from(&self, other: &Particle) -> Option<Vector> {
        let diff = self.location - other.location;
        let distance_squared = diff.norm_squared();
        if distance_squared == 0. {
            return None;
        }
        let magnitude = 1. / distance_squared;
        assert!(magnitude.is_finite() && magnitude > 0.);
        let result = diff.normalize() * magnitude;
        assert!(result.x.is_finite() && result.y.is_finite());
        Some(result)
    }

    pub fn air_resistance(&self) -> Vector {
        -self.velocity * self.velocity.norm()
    }

    pub fn rebound(&mut self, x_limits: (Float, Float), y_limits: (Float, Float)) {
        let (x_min, x_max) = x_limits;
        let (y_min, y_max) = y_limits;

        if self.location.x < x_min {
            self.location.x = 2. * x_min - self.location.x;
            self.velocity.x *= -1.;
        } else if self.location.x > x_max {
            self.location.x = 2. * x_max - self.location.x;
            self.velocity.x *= -1.;
        }
        assert!(
            self.location.x >= x_min && self.location.x <= x_max,
            "{self:?}"
        );

        if self.location.y < y_min {
            self.location.y = 2. * y_min - self.location.y;
            self.velocity.y *= -1.;
        } else if self.location.y > y_max {
            self.location.y = 2. * y_max - self.location.y;
            self.velocity.y *= -1.;
        }
        assert!(
            self.location.y >= y_min && self.location.y <= y_max,
            "{self:?}"
        );
    }
}
