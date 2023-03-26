use crate::vec3::Point3;

// Image
pub const ASCPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: u32 = 400;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASCPECT_RATIO) as u32;
pub const PROGRESS_NUM_WIDTH: u32 = IMAGE_HEIGHT.ilog10() + 1;

// Rendering
pub const SAMPLES_PER_PIXEL: u32 = 100;
pub const MAX_DEPTH: i32 = 50;

// Camera
pub const LOOK_FROM: Point3 = Point3::new(-2.0, 2.0, 1.0);
pub const LOOK_AT: Point3 = Point3::new(0.0, 0.0, -1.0);
pub const VIEW_UP: Point3 = Point3::new(0.0, 1.0, 0.0);
pub const VERTICAL_FOV: f64 = 90.0;  // in degrees
pub const FOCAL_LENGTH: f64 = 1.0;

// File format
pub const FILE_TYPE: &str = "P3";
pub const MAX_COLOR: u32 = 255;
