use std::ops::{Add, AddAssign, Sub};

use simulation::Vector;

#[derive(Clone, Copy, Debug)]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32,
}

impl ScreenPosition {
    pub fn from_tuple((x, y): (f32, f32)) -> Self {
        Self { x, y }
    }
}

impl AddAssign<Vector> for ScreenPosition {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Vector> for ScreenPosition {
    type Output = ScreenPosition;

    fn add(mut self, rhs: Vector) -> ScreenPosition {
        self += rhs;
        self
    }
}

impl Sub<ScreenPosition> for ScreenPosition {
    type Output = Vector;

    fn sub(self, other: ScreenPosition) -> Vector {
        Vector::new(self.x - other.x, self.y - other.y)
    }
}

impl From<Vector> for ScreenPosition {
    fn from(vector: Vector) -> Self {
        Self {
            x: vector.x,
            y: vector.y,
        }
    }
}

impl From<ScreenPosition> for Vector {
    fn from(screen_position: ScreenPosition) -> Self {
        Self {
            x: screen_position.x,
            y: screen_position.y,
        }
    }
}
