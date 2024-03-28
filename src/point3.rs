#![allow(dead_code)]

use crate::vec3::Vec3;

use std::ops::Sub;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Point3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Point3 { x, y, z }
    }

    pub fn zero() -> Self {
        Point3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.x += translation.x;
        self.y += translation.y;
        self.z += translation.z;
    }

    pub fn translated(self, translation: Vec3) -> Point3 {
        Point3::new(
            self.x + translation.x,
            self.y + translation.y,
            self.z + translation.z,
        )
    }

    pub fn distance(self, other: Point3) -> f32 {
        (self - other).length()
    }

    pub fn distance_squared(self, other: Point3) -> f32 {
        (self - other).length_squared()
    }
}

impl Sub for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
