use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};
use crate::config::{ASCPECT_RATIO, VERTICAL_FOV, LOOK_FROM, LOOK_AT, VIEW_UP};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let theta = VERTICAL_FOV.to_radians();
        let h = f64::sin(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = ASCPECT_RATIO * viewport_height;

        let w = (LOOK_FROM - LOOK_AT).normalized();
        let u = Vec3::cross(VIEW_UP, w).normalized();
        let v = Vec3::cross(w, u);

        let origin = LOOK_FROM;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
