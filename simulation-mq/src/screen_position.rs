use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

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

#[derive(Clone, Copy, Debug)]
pub struct ScreenVector {
    pub x: f32,
    pub y: f32,
}

impl ScreenVector {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl AddAssign<ScreenVector> for ScreenPosition {
    fn add_assign(&mut self, rhs: ScreenVector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<ScreenVector> for ScreenPosition {
    type Output = ScreenPosition;

    fn add(mut self, rhs: ScreenVector) -> ScreenPosition {
        self += rhs;
        self
    }
}

impl AddAssign<ScreenVector> for ScreenVector {
    fn add_assign(&mut self, rhs: ScreenVector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<ScreenVector> for ScreenVector {
    type Output = ScreenVector;

    fn add(mut self, rhs: ScreenVector) -> ScreenVector {
        self += rhs;
        self
    }
}

impl Sub<ScreenPosition> for ScreenPosition {
    type Output = ScreenVector;

    fn sub(self, other: ScreenPosition) -> ScreenVector {
        ScreenVector::new(self.x - other.x, self.y - other.y)
    }
}

impl SubAssign<ScreenVector> for ScreenPosition {
    fn sub_assign(&mut self, rhs: ScreenVector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl SubAssign<ScreenVector> for ScreenVector {
    fn sub_assign(&mut self, rhs: ScreenVector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<ScreenVector> for ScreenVector {
    type Output = ScreenVector;

    fn sub(mut self, other: ScreenVector) -> ScreenVector {
        self -= other;
        self
    }
}

impl MulAssign<f32> for ScreenVector {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<f32> for ScreenVector {
    type Output = ScreenVector;

    fn mul(mut self, rhs: f32) -> ScreenVector {
        self *= rhs;
        self
    }
}

impl Mul<ScreenVector> for f32 {
    type Output = ScreenVector;

    fn mul(self, rhs: ScreenVector) -> ScreenVector {
        rhs * self
    }
}

impl From<ScreenVector> for ScreenPosition {
    fn from(screen_vector: ScreenVector) -> Self {
        Self {
            x: screen_vector.x,
            y: screen_vector.y,
        }
    }
}

impl From<ScreenPosition> for ScreenVector {
    fn from(screen_position: ScreenPosition) -> Self {
        Self {
            x: screen_position.x,
            y: screen_position.y,
        }
    }
}

impl From<Vector> for ScreenVector {
    fn from(vector: Vector) -> Self {
        Self {
            x: vector.x as f32,
            y: vector.y as f32,
        }
    }
}
