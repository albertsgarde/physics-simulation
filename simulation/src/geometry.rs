use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use delegate::delegate;
use nalgebra::Vector2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Location {
    loc: Vector2<f32>,
}

impl Location {
    pub fn new(x: f32, y: f32) -> Self {
        Location {
            loc: Vector2::new(x, y),
        }
    }

    pub fn x(&self) -> f32 {
        self.loc.x
    }

    pub fn y(&self) -> f32 {
        self.loc.y
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Vector {
    vec: Vector2<f32>,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Vector {
            vec: Vector2::new(x, y),
        }
    }

    pub fn x(&self) -> f32 {
        self.vec.x
    }

    pub fn y(&self) -> f32 {
        self.vec.y
    }

    delegate!(to self.vec {
        pub fn norm(&self) -> f32;
        pub fn norm_squared(&self) -> f32;
        #[into]
        pub fn normalize(&self) -> Vector;
    });

    pub fn dot(&self, other: Vector) -> f32 {
        self.vec.dot(&other.vec)
    }
}

impl Add<Vector> for Location {
    type Output = Location;

    fn add(self, rhs: Vector) -> Location {
        Location {
            loc: self.loc + rhs.vec,
        }
    }
}

impl AddAssign<Vector> for Location {
    fn add_assign(&mut self, rhs: Vector) {
        self.loc += rhs.vec;
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Vector {
        Vector {
            vec: self.vec + rhs.vec,
        }
    }
}

impl AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.vec += rhs.vec;
    }
}

impl Sub<Vector> for Location {
    type Output = Location;

    fn sub(mut self, rhs: Vector) -> Location {
        self -= rhs;
        self
    }
}

impl SubAssign<Vector> for Location {
    fn sub_assign(&mut self, rhs: Vector) {
        self.loc -= rhs.vec;
    }
}

impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, rhs: Vector) -> Vector {
        Vector {
            vec: self.vec - rhs.vec,
        }
    }
}

impl SubAssign<Vector> for Vector {
    fn sub_assign(&mut self, rhs: Vector) {
        self.vec -= rhs.vec;
    }
}

impl Sub<Location> for Location {
    type Output = Vector;

    fn sub(self, rhs: Location) -> Vector {
        Vector {
            vec: self.loc - rhs.loc,
        }
    }
}

impl Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f32) -> Vector {
        Vector {
            vec: self.vec * rhs,
        }
    }
}

impl MulAssign<f32> for Vector {
    fn mul_assign(&mut self, rhs: f32) {
        self.vec *= rhs;
    }
}

impl Mul<Vector> for f32 {
    type Output = Vector;

    fn mul(self, rhs: Vector) -> Vector {
        Vector {
            vec: self * rhs.vec,
        }
    }
}

impl Div<f32> for Vector {
    type Output = Vector;

    fn div(self, rhs: f32) -> Vector {
        Vector {
            vec: self.vec / rhs,
        }
    }
}

impl DivAssign<f32> for Vector {
    fn div_assign(&mut self, rhs: f32) {
        self.vec /= rhs;
    }
}

impl From<Vector> for Vector2<f32> {
    fn from(v: Vector) -> Vector2<f32> {
        v.vec
    }
}

impl From<Vector2<f32>> for Vector {
    fn from(v: Vector2<f32>) -> Vector {
        Vector { vec: v }
    }
}