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
        let (viewport_width, viewport_height) = calculate_viewport_dimensions();
        let (u, v, w) = calculate_camera_orientation();

        let lens_radius = APERTURE / 2.0;
        let focus_distance = match FOCUS_DISTANCE {
            Some(distance) => distance,
            None => (LOOK_FROM - LOOK_AT).length(),
        };

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

        let start = self.origin + offset;
        let point_on_focus_plane = self.lower_left_corner + s * self.horizontal + t * self.vertical;
        let direction = point_on_focus_plane - start;

        Ray::new(start, direction)
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

fn calculate_viewport_dimensions() -> (f64, f64) {
    let vertical_radians = VERTICAL_FOV.to_radians();
    let half_vertical_distance_on_viewing_plane = f64::tan(vertical_radians / 2.0);
    let viewport_height = 2.0 * half_vertical_distance_on_viewing_plane;
    let viewport_width = ASCPECT_RATIO * viewport_height;

    (viewport_width, viewport_height)
}

fn calculate_camera_orientation() -> (Vec3, Vec3, Vec3) {
    let view_direction = LOOK_AT - LOOK_FROM;
    let w = (-view_direction).normalized();
    let u = Vec3::cross(VIEW_UP, w).normalized();
    let v = Vec3::cross(w, u);

    (u, v, w)
}
