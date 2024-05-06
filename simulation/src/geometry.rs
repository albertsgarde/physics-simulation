use std::{
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::Float;

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Location {
    pub x: Float,
    pub y: Float,
}

impl Location {
    pub const ORIGIN: Location = Location { x: 0., y: 0. };

    pub fn new(x: Float, y: Float) -> Self {
        Location { x, y }
    }

    pub fn dist_squared(self, other: Location) -> Float {
        (self - other).norm_squared()
    }

    pub fn dist(self, other: Location) -> Float {
        (self - other).norm()
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Vector {
    pub x: Float,
    pub y: Float,
}

impl Vector {
    pub const ZERO: Vector = Vector { x: 0., y: 0. };

    pub fn new(x: Float, y: Float) -> Self {
        Vector { x, y }
    }

    pub fn dot(&self, other: Vector) -> Float {
        self.x * other.x + self.y * other.y
    }

    pub fn norm_squared(self) -> Float {
        self.dot(self)
    }

    pub fn norm(self) -> Float {
        self.norm_squared().sqrt()
    }

    pub fn normalize(self) -> Self {
        self / self.norm()
    }

    pub fn random_unit(rng: &mut impl Rng) -> Self {
        let angle = rng.gen_range(0. ..2. * crate::PI);
        Vector::new(angle.cos(), angle.sin())
    }
}

impl AddAssign<Vector> for Location {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Vector> for Location {
    type Output = Location;

    fn add(mut self, rhs: Vector) -> Location {
        self += rhs;
        self
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(mut self, rhs: Vector) -> Vector {
        self += rhs;
        self
    }
}

impl SubAssign<Vector> for Location {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<Vector> for Location {
    type Output = Location;

    fn sub(mut self, rhs: Vector) -> Location {
        self -= rhs;
        self
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Sub<Location> for Location {
    type Output = Vector;

    fn sub(self, rhs: Location) -> Vector {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl MulAssign<Float> for Vector {
    fn mul_assign(&mut self, rhs: Float) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<Float> for Vector {
    type Output = Vector;

    fn mul(mut self, rhs: Float) -> Vector {
        self *= rhs;
        self
    }
}

impl Mul<Vector> for Float {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl DivAssign<Float> for Vector {
    fn div_assign(&mut self, rhs: Float) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl Div<Float> for Vector {
    type Output = Vector;

    fn div(mut self, rhs: Float) -> Vector {
        self /= rhs;
        self
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl Sum for Vector {
    fn sum<I: Iterator<Item = Vector>>(iter: I) -> Self {
        iter.fold(Vector::new(0., 0.), Add::add)
    }
}

impl From<Location> for Vector {
    fn from(location: Location) -> Vector {
        Vector {
            x: location.x,
            y: location.y,
        }
    }
}
