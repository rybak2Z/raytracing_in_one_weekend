mod camera_builder;

pub use camera_builder::CameraBuilder;

use crate::random::random_range;
use crate::{Point3, Ray, Vec3};

#[derive(Clone)]
struct Orientation {
    #[allow(dead_code)]
    look_at: Point3,
    #[allow(dead_code)]
    view_up: Vec3,
}

#[derive(Clone)]
struct Basis {
    up: Vec3,
    right: Vec3,
    back: Vec3,
}

#[derive(Clone)]
struct Defocus {
    #[allow(dead_code)]
    focus_distance: f32,
    angle: f32,
    disk_u: Vec3,
    disk_v: Vec3,
}

impl Defocus {
    fn disk_sample(&self, position: Point3) -> Point3 {
        let offset = Vec3::random_in_unit_disk();
        position + (offset.x * self.disk_u) + (offset.y * self.disk_v)
    }
}

#[derive(Clone)]
struct Image {
    width: u32,
    height: u32,
    #[allow(dead_code)]
    aspect_ratio: f32,
}

#[derive(Clone)]
struct Viewport {
    #[allow(dead_code)]
    width: f32,
    #[allow(dead_code)]
    height: f32,
    pixel_top_left: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Viewport {
    pub fn pixel_sample(&self, row: u32, column: u32) -> Point3 {
        let offset_x = column as f32 * self.pixel_delta_u;
        let offset_y = row as f32 * self.pixel_delta_v;
        let pixel_center = self.pixel_top_left + offset_x + offset_y;
        pixel_center + self.random_pixel_square_sample()
    }

    fn random_pixel_square_sample(&self) -> Vec3 {
        let factor_x = random_range(-0.5, 0.5);
        let factor_y = random_range(-0.5, 0.5);
        (factor_x * self.pixel_delta_u) + (factor_y * self.pixel_delta_v)
    }
}

#[derive(Clone)]
pub struct Camera {
    position: Point3,
    #[allow(dead_code)]
    vertical_fov: f32,
    #[allow(dead_code)]
    orientation: Orientation,
    #[allow(dead_code)]
    basis: Basis,
    defocus: Defocus,
    image: Image,
    viewport: Viewport,
}

impl Camera {
    pub fn get_ray(&self, row: u32, column: u32) -> Ray {
        let pixel_sample = self.viewport.pixel_sample(row, column);

        let origin = match self.defocus.angle <= 0.0 {
            true => self.position,
            false => self.defocus.disk_sample(self.position),
        };
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }

    pub fn image_width(&self) -> u32 {
        self.image.width
    }

    pub fn image_height(&self) -> u32 {
        self.image.height
    }
}
