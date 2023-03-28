use crate::material::Material;
use crate::ray::Ray;
use crate::rendering::*;
use crate::vec3::*;

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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center_to_origin = ray.origin() - self.center;

        // Quadratic equation
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(center_to_origin, ray.direction());
        let c = center_to_origin.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        let solution = find_smallest_valid_solution(a, half_b, discriminant, t_min, t_max)?;

        let mut record = HitRecord::default();
        record.t = solution;
        record.point = ray.at(record.t);
        let outward_normal = (record.point - self.center) / self.radius;
        record.set_face_normal(ray, outward_normal);
        record.material = Rc::clone(&self.material);

        Some(record)
    }
}

fn find_smallest_valid_solution(a: f64, half_b: f64, discriminant: f64, t_min: f64, t_max: f64) -> Option<f64> {
    if discriminant < 0.0 {
        return None;
    }

    let sqrt_discriminant = discriminant.sqrt();
    let mut root = (-half_b - sqrt_discriminant) / a;
    if root < t_min || t_max < root {
        root = (-half_b + sqrt_discriminant) / a;
        if root < t_min || t_max < root {
            return None;
        }
    }

    Some(root)
}
