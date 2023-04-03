use super::Vec3;

use std::ops;

#[derive(Debug, Clone, Copy, Default)]
pub struct Point3 {
    coords: [f64; 3],
}

impl Point3 {
    pub const fn new(x: f64, y: f64, z: f64) -> Point3 {
        Point3 { coords: [x, y, z] }
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

    pub fn get(&self, dimension: usize) -> f64 {
        self.coords[dimension]
    }

    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        self.x().abs() < s && self.y().abs() < s && self.z().abs() < s
    }

    pub fn pos_vec(&self) -> Vec3 {
        // position vector
        Vec3::new(self.x(), self.y(), self.z())
    }

    pub fn random() -> Point3 {
        let random_vec = Vec3::random();
        Point3::new(random_vec.x(), random_vec.y(), random_vec.z())
    }

    pub fn random_range(min: f64, max: f64) -> Point3 {
        let random_vec = Vec3::random_range(min, max);
        Point3::new(random_vec.x(), random_vec.y(), random_vec.z())
    }

    pub fn random_in_unit_sphere() -> Point3 {
        let random_vec = Vec3::random_in_unit_sphere();
        Point3::new(random_vec.x(), random_vec.y(), random_vec.z())
    }

    pub fn random_in_hemisphere(normal: Vec3) -> Point3 {
        let random_vec = Vec3::random_in_hemisphere(normal);
        Point3::new(random_vec.x(), random_vec.y(), random_vec.z())
    }

    pub fn random_in_unit_disk() -> Point3 {
        let random_vec = Vec3::random_in_unit_disk();
        Point3::new(random_vec.x(), random_vec.y(), random_vec.z())
    }
}

impl ops::Add for Point3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Point3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::Add<Vec3> for Point3 {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl ops::AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl ops::Sub for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl ops::Sub<Vec3> for Point3 {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        self + (-rhs)
    }
}

impl ops::SubAssign<Vec3> for Point3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self += -rhs;
    }
}
