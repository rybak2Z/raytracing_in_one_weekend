use super::{Material, Scatter};

use crate::{Color, HitRecord, Ray, Vec3};

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit_rec: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = hit_rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = hit_rec.normal;
        }

        let scattered = Ray::new(hit_rec.point, scatter_direction);
        let scatter = Scatter::new(scattered, self.albedo);

        Some(scatter)
    }
}
