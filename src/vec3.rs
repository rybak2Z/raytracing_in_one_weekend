use rand::prelude::*;
use std::{fmt::Display, ops};

pub type Point3 = Vec3;
pub type Color = Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    coords: [f64; 3],
}

impl Vec3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { coords: [x, y, z] }
    }

    pub fn x(&self) -> f64 {
        self.coords[0]
    }

    pub fn y(&self) -> f64 {
        self.coords[1]
    }

    pub fn z(&self) -> f64 {
        self.coords[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn normalized(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn dot(v1: Vec3, v2: Vec3) -> f64 {
        v1.x() * v2.x() + v1.y() * v2.y() + v1.z() * v2.z()
    }

    pub fn cross(v1: Vec3, v2: Vec3) -> Vec3 {
        Vec3::new(
            v1.y() * v2.z() - v1.z() * v2.y(),
            v1.z() * v2.x() - v1.x() * v2.z(),
            v1.x() * v2.y() - v1.y() * v2.x(),
        )
    }

    pub fn random() -> Vec3 {
        let mut rng = thread_rng();
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        let mut rng = thread_rng();
        Vec3::new(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        let mut point = Vec3::random_range(-1.0, 1.0);
        while point.length_squared() >= 1.0 {
            point = Vec3::random_range(-1.0, 1.0);
        }
        point
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().normalized()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x(), self.y(), self.z())
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x(), -self.y(), -self.z())
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() / rhs, self.y() / rhs, self.z() / rhs)
    }
}

impl ops::Div<Vec3> for f64 {
    type Output = Vec3;

    fn div(self, rhs: Vec3) -> Self::Output {
        rhs / self
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0 / rhs;
    }
}
