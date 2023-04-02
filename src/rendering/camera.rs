use crate::config::ASPECT_RATIO;
use crate::rendering::{Point3, Ray, Vec3};

pub struct CameraConfiguration {
    pub look_from: Point3,
    pub look_at: Point3,
    pub view_up: Vec3,
    pub vertical_fov: f64,
    pub aperture: f64,
    pub focus_distance: Option<f64>, // if None, the distance between look_from and look_at will be used
    pub focal_length: f64,
}

#[derive(Clone)]
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
    pub fn new(config: CameraConfiguration) -> Camera {
        let (viewport_width, viewport_height) = calculate_viewport_dimensions(config.vertical_fov);
        let (u, v, w) =
            calculate_camera_orientation(config.look_from, config.look_at, config.view_up);

        let lens_radius = config.aperture / 2.0;
        let focus_distance = match config.focus_distance {
            Some(distance) => distance,
            None => (config.look_from - config.look_at).length(),
        };

        let origin = config.look_from;
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

fn calculate_viewport_dimensions(vertical_fov: f64) -> (f64, f64) {
    let vertical_radians = vertical_fov.to_radians();
    let half_vertical_distance_on_viewing_plane = f64::tan(vertical_radians / 2.0);
    let viewport_height = 2.0 * half_vertical_distance_on_viewing_plane;
    let viewport_width = ASPECT_RATIO.get().unwrap() * viewport_height;

    (viewport_width, viewport_height)
}

fn calculate_camera_orientation(
    look_from: Point3,
    look_at: Point3,
    view_up: Vec3,
) -> (Vec3, Vec3, Vec3) {
    let view_direction = look_at - look_from;
    let w = (-view_direction).normalized();
    let u = Vec3::cross(view_up, w).normalized();
    let v = Vec3::cross(w, u);

    (u, v, w)
}
