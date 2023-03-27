use rand::prelude::*;
pub use std::rc::Rc;

use crate::camera::Camera;
use crate::config::*;
use crate::ray::*;
use crate::vec3::*;
use crate::writing::*;

pub fn render(
    world: &HittableList,
    writer: &mut BufWriter<StdoutLock>,
    writer_err: &mut BufWriter<StderrLock>,
) -> io::Result<()> {
    let cam = Camera::new();
    let mut rng = thread_rng();

    for row in (0..IMAGE_HEIGHT).rev() {
        write_progress_update(row, writer_err)?;
        for col in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for _sample in 0..SAMPLES_PER_PIXEL {
                let u = (col as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (row as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = cam.get_ray(u, v);
                pixel_color += get_ray_color(ray, world, MAX_DEPTH);
            }
            write_pixel(writer, pixel_color)?;
        }
    }

    Ok(())
}

fn get_ray_color(ray: Ray, world: &HittableList, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::default();
    }

    let mut hit_record = HitRecord::new();
    if world.hit(ray, 0.0001, f64::INFINITY, &mut hit_record) {
        let mut scattered_ray = Ray::default();
        let mut attenuation = Color::default();
        if hit_record
            .material
            .scatter(ray, &hit_record, &mut attenuation, &mut scattered_ray)
        {
            return attenuation * get_ray_color(scattered_ray, world, depth - 1);
        }
        return Color::default();
    }

    let direction = ray.direction().normalized();
    let t = 0.5 * (direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

pub struct HitRecord {
    point: Point3,
    normal: Vec3,
    material: Rc<dyn Material>,
    t: f64,
    on_front_face: bool,
}

impl HitRecord {
    fn new() -> HitRecord {
        HitRecord {
            point: Point3::default(),
            normal: Vec3::default(),
            material: Rc::new(Lambertian::new(Color::default())),
            t: 0.0,
            on_front_face: true,
        }
    }

    fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.on_front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = match self.on_front_face {
            true => outward_normal,
            false => -outward_normal,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let co = ray.origin() - self.center;

        // Quadratic equation
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(co, ray.direction());
        let c = co.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return false;
        }

        // Find the nearest root that lies in the qacceptable range
        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (-half_b - sqrt_discriminant) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrt_discriminant) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        record.t = root;
        record.point = ray.at(record.t);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        record.material = Rc::clone(&self.material);

        true
    }
}

pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest = HitRecord::new();
        closest.t = t_max;

        for object in self.objects.iter() {
            if object.hit(ray, t_min, t_max, &mut temp_record) && temp_record.t < closest.t {
                hit_anything = true;
                std::mem::swap(&mut temp_record, &mut closest);
            }
        }
        if hit_anything {
            *record = closest;
        }

        hit_anything
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Material {
    fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool;
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
    fn scatter(
        &self,
        _ray_in: Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered_ray = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;

        true
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
    fn scatter(
        &self,
        _ray_in: Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        let mut scatter_direction = Vec3::random_in_hemisphere(hit_record.normal);

        // Catch degenerate scatter direction
        if scatter_direction.is_near_zero() {
            scatter_direction = hit_record.normal;
        }

        *scattered_ray = Ray::new(hit_record.point, scatter_direction);
        *attenuation = self.albedo;

        true
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
    fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        let reflected_direction = Vec3::reflect(ray_in.direction(), hit_record.normal);
        *scattered_ray = Ray::new(
            hit_record.point,
            reflected_direction + self.fuzziness * Vec3::random_in_unit_sphere(),
        );
        *attenuation = self.albedo;

        Vec3::dot(scattered_ray.direction(), hit_record.normal) > 0.0
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
    fn scatter(
        &self,
        ray_in: Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        scattered_ray: &mut Ray,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
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

        *scattered_ray = Ray::new(hit_record.point, direction);

        true
    }
}
