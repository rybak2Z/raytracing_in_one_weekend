use super::{Material, Scatter};

use crate::{Color, HitRecord, Ray, Vec3};

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Self {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let mut reflected = ray.direction().normalized().reflected(hit_record.normal);
        reflected += self.fuzz * Vec3::random_in_unit_sphere();

        if Vec3::dot(reflected, hit_record.normal) <= 0.0 {
            return None;
        }

        let scattered = Ray::new(hit_record.point, reflected);
        let scatter = Scatter::new(scattered, self.albedo);
        Some(scatter)
    }
}
