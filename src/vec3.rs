#![allow(dead_code)]

use crate::random;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(Default, Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Constructors
impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn zero() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
}

/// Properties
impl Vec3 {
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn near_zero(self) -> bool {
        let e = 1e-8;
        self.x.abs() < e && self.y.abs() < e && self.z.abs() < e
    }
}

/// Mutations
impl Vec3 {
    pub fn normalize(&mut self) {
        *self /= self.length();
    }
}

/// Transformations
impl Vec3 {
    pub fn normalized(self) -> Self {
        self / self.length()
    }

    pub fn reflected(self, normal: Vec3) -> Self {
        self - 2.0 * Self::dot(self, normal) * normal
    }

    pub fn refracted(self, normal: Vec3, etai_over_etat: f32) -> Self {
        let cos_theta = Self::dot(-self, normal).min(1.0);
        let perpendicular_part = etai_over_etat * (self + cos_theta * normal);
        let parallel_part = -(1.0 - perpendicular_part.length_squared()).abs().sqrt() * normal;
        perpendicular_part + parallel_part
    }
}

/// Other operations
impl Vec3 {
    pub fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
        lhs.x * rhs.x + lhs.y * rhs.y + lhs.z * rhs.z
    }

    pub fn cross(lhs: Vec3, rhs: Vec3) -> Self {
        Self::new(
            lhs.y * rhs.z - lhs.z * rhs.y,
            lhs.z * rhs.x - lhs.x * rhs.z,
            lhs.x * rhs.y - lhs.y * rhs.x,
        )
    }
}

/// Constructors for randomized vectors
impl Vec3 {
    pub fn random() -> Self {
        Self {
            x: random::random(),
            y: random::random(),
            z: random::random(),
        }
    }

    pub fn random_range(min: f32, max: f32) -> Self {
        let x = random::random_range(min, max);
        let y = random::random_range(min, max);
        let z = random::random_range(min, max);
        Self { x, y, z }
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut vector = Self::random_range(-1.0, 1.0);
        while vector.length_squared() > 1.0 {
            vector = Self::random_range(-1.0, 1.0);
        }
        vector
    }

    pub fn random_unit_vector() -> Self {
        let mut in_unit_sphere = Self::random_in_unit_sphere();
        in_unit_sphere.normalize();
        in_unit_sphere
    }

    pub fn random_on_hemisphere(normal: Vec3) -> Self {
        let on_unit_sphere = Self::random_unit_vector();
        match Self::dot(on_unit_sphere, normal) > 0.0 {
            true => on_unit_sphere,
            false => -on_unit_sphere,
        }
    }

    pub fn random_in_unit_disk() -> Self {
        let random_in_unit_square = || {
            Self::new(
                random::random_range(-1.0, 1.0),
                random::random_range(-1.0, 1.0),
                0.0,
            )
        };

        let mut vector = random_in_unit_square();
        while vector.length_squared() > 1.0 {
            vector = random_in_unit_square();
        }

        vector
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self * (-1.0)
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + (-rhs)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        *self += -rhs;
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}
