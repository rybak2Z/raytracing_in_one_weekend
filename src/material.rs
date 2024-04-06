mod dialectric;
mod lambertian;
mod metal;

pub use dialectric::Dialectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{Color, HitRecord, Ray};

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

impl Scatter {
    pub fn new(ray: Ray, attenuation: Color) -> Scatter {
        Scatter { ray, attenuation }
    }
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}
