use crate::{HitRecord, Hittable, Interval, Material, Point3, Ray, Vec3};

use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f32,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn center(&self) -> Point3 {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

impl Hittable for Sphere {
    fn hit_mutating(&self, ray: &Ray, allowed_t: Interval, hit_rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrt_discriminant = discriminant.sqrt();

        // Find the nearest t that lies in the acceptable range
        let mut t = (-half_b - sqrt_discriminant) / a;
        if !allowed_t.surrounds(t) {
            t = (-half_b + sqrt_discriminant) / a;
            if !allowed_t.surrounds(t) {
                return false;
            }
        }

        hit_rec.t = t;
        hit_rec.point = ray.at(t);
        let outward_normal = (hit_rec.point - self.center) / self.radius;
        hit_rec.set_face_normal(ray, outward_normal);
        hit_rec.material = Some(Rc::clone(&self.material));

        true
    }
}
