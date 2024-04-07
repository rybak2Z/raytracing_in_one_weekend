pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod point3;
pub mod random;
pub mod ray;
pub mod renderer;
pub mod sphere;
pub mod vec3;
pub mod writing;

use std::sync::Arc;

pub use color::Color;
pub use hittable::{HitRecord, Hittable};
pub use interval::Interval;
pub use material::{Dialectric, Lambertian, Material, Metal, Scatter};
pub use point3::Point3;
pub use ray::Ray;
pub use renderer::Renderer;
pub use vec3::Vec3;

pub type SharedMaterial = Arc<dyn Material + Send + Sync>;
type SharedHittable = Arc<dyn Hittable + Send + Sync>;
