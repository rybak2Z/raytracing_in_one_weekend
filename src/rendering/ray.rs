use super::{Point3, Vec3};

#[derive(Debug, Clone, Copy, Default)]
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3, time: Option<f64>) -> Ray {
        Ray { origin, direction, time: time.unwrap_or(0.0) }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
