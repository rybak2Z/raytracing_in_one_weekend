pub mod color;
pub mod point3;
pub mod ray;
pub mod vec3;

pub use color::Color;
pub use point3::Point3;
pub use ray::Ray;
pub use vec3::Vec3;

pub const FILE_TYPE: &str = "P3";
pub const MAX_VALUE: u32 = 255;
pub const IMAGE_WIDTH: u32 = 256;
pub const IMAGE_HEIGHT: u32 = 256;
