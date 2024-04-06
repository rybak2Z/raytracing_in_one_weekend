use super::{Material, Scatter};

use crate::{random, Color, HitRecord, Ray, Vec3};

pub struct Dialectric {
    refraction: f32,
}

impl Dialectric {
    pub fn new(index_of_refraction: f32) -> Dialectric {
        Dialectric {
            refraction: index_of_refraction,
        }
    }

    fn reflectance(&self, cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = match hit_record.front_face {
            true => 1.0 / self.refraction,
            false => self.refraction,
        };

        let normalized_direction = ray.direction().normalized();
        let cos_theta = Vec3::dot(-normalized_direction, hit_record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflects =
            cannot_refract || self.reflectance(cos_theta, refraction_ratio) > random::random();
        let direction = match reflects {
            true => normalized_direction.reflected(hit_record.normal),
            false => normalized_direction.refracted(hit_record.normal, refraction_ratio),
        };

        let scattered_ray = Ray::new(hit_record.point, direction);
        let attenuation = Color::new(1.0, 1.0, 1.0);

        Some(Scatter::new(scattered_ray, attenuation))
    }
}
