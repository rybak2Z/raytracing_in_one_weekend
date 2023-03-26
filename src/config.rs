use crate::vec3::{Point3, Vec3};

// Image
pub const ASCPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: u32 = 400;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASCPECT_RATIO) as u32;

// Camera
pub const VIEWPORT_HEIGHT: f64 = 2.0;
pub const VIEWPORT_WIDTH: f64 = ASCPECT_RATIO * VIEWPORT_HEIGHT;
pub const FOCAL_LENGTH: f64 = 1.0;
pub const ORIGIN: Point3 = Point3::new(0.0, 0.0, 0.0);
pub const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
pub const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);

// File format
pub const FILE_TYPE: &str = "P3";
pub const MAX_COLOR: u32 = 255;

// I'd rather have this as a constant but computing it at compile-time is not allowed
pub fn compute_lower_left_corner() -> Point3 {
    ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH)
}
