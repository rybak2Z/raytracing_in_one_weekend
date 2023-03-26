use crate::config::{
    APERTURE, ASCPECT_RATIO, FOCUS_DISTANCE, LOOK_AT, LOOK_FROM, VERTICAL_FOV, VIEW_UP,
};
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f64,
}

impl Camera {
    pub fn new() -> Camera {
        let theta = VERTICAL_FOV.to_radians();
        let h = f64::sin(theta / 2.0);
        let viewport_height = 2.0 * h;
        let viewport_width = ASCPECT_RATIO * viewport_height;
        let lens_radius = APERTURE / 2.0;

        let focus_distance = match FOCUS_DISTANCE {
            Some(distance) => distance,
            None => (LOOK_FROM - LOOK_AT).length(),
        };

        let w = (LOOK_FROM - LOOK_AT).normalized();
        let u = Vec3::cross(VIEW_UP, w).normalized();
        let v = Vec3::cross(w, u);

        let origin = LOOK_FROM;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_distance * w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            u,
            v,
            _w: w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let radius = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.u * radius.x() + self.v * radius.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}
