use crate::{Color, HitRecord, Ray, Vec3};

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

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
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

pub struct Metal {
    albedo: Color,
    fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f32) -> Metal {
        let fuzz = if fuzz < 1.0 { fuzz } else { 1.0 };
        Metal { albedo, fuzz }
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
