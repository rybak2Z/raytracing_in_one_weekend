use crate::rendering::Method;

// Image
pub const ASCPECT_RATIO: f64 = 16.0 / 9.0;
pub const IMAGE_WIDTH: u32 = 400;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASCPECT_RATIO) as u32;
pub const PROGRESS_NUM_WIDTH: u32 = IMAGE_HEIGHT.ilog10() + 1;

// Rendering
pub const SAMPLES_PER_PIXEL: u32 = 100;
pub const MAX_DEPTH: i32 = 50;
pub const METHOD: Method = Method::Lambertian;

// File format
pub const FILE_TYPE: &str = "P3";
pub const MAX_COLOR: u32 = 255;
