use crate::ray::Ray;
use crate::rendering::HitRecord;
use crate::vec3::{Color, Vec3};

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered_ray = Ray::new(hit_record.point, scatter_direction);

        Some(Scatter { ray: scattered_ray, attenuation: self.albedo })
    }
}

pub struct UniformScatter {
    albedo: Color,
}

impl UniformScatter {
    pub fn new(albedo: Color) -> UniformScatter {
        UniformScatter { albedo }
    }
}

impl Material for UniformScatter {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = Vec3::random_in_hemisphere(hit_record.normal);

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        let scattered_ray = Ray::new(hit_record.point, scatter_direction);

        Some(Scatter { ray: scattered_ray, attenuation: self.albedo })
    }
}

pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Metal {
        let fuzziness = fuzziness.min(1.0);
        Metal { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let reflected_direction = Vec3::reflect(ray_in.direction(), hit_record.normal);
        let scattered_ray = Ray::new(
            hit_record.point,
            reflected_direction + self.fuzziness * Vec3::random_in_unit_sphere(),
        );
        let does_hit = Vec3::dot(scattered_ray.direction(), hit_record.normal) > 0.0;
        if !does_hit {
            return None;
        }

        Some(Scatter { ray: scattered_ray, attenuation: self.albedo })
    }
}

pub struct Dialectric {
    refractive_index: f64,
}

impl Dialectric {
    pub fn new(refractive_index: f64) -> Dialectric {
        Dialectric { refractive_index }
    }
}

impl Material for Dialectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let refraction_ratio = match hit_record.on_front_face {
            true => 1.0 / self.refractive_index,
            false => self.refractive_index,
        };

        let unit_direction = ray_in.direction().normalized();
        let cos_theta = 1.0_f64.min(Vec3::dot(-unit_direction, hit_record.normal));
        let sin_theta = (1.0_f64 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            Vec3::reflect(unit_direction, hit_record.normal)
        } else {
            Vec3::refract(unit_direction, hit_record.normal, refraction_ratio)
        };

        let scattered_ray = Ray::new(hit_record.point, direction);
        let attenuation = Color::new(1.0, 1.0, 1.0);

        Some(Scatter { ray: scattered_ray, attenuation })
    }
}
