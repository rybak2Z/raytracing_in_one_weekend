use super::{
    hit_detection::{HitRecord, Hittable},
    Material, Point3, Ray, Vec3, AABB,
};

use std::f64::consts::PI;

#[derive(Clone)]
pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Box<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Box<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    pub fn get_sphere_uv(point: Point3) -> (f64, f64) {
        // p: a given point on the unit sphere, centered at hte origin
        // returns (u, v) with
        // u: value in [0, 1] for angle around y axis starting from x=-1
        // v: vlaue in [0, 1] for angle from y=-1 to y=+1

        let theta: f64 = (-point.y()).acos();
        let phi: f64 = (-point.z()).atan2(point.x()) + PI;

        (phi / (2.0 * PI), theta / PI)
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

        let hit_point = ray.at(solution);
        let outward_normal = (hit_point - self.center) / self.radius;
        let (u, v) = Sphere::get_sphere_uv(hit_point);
        let record = HitRecord::new(ray, solution, u, v, outward_normal, self.material.clone());

        Some(record)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let offset = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center - offset, self.center + offset))
    }
}

pub fn find_smallest_valid_solution(
    a: f64,
    half_b: f64,
    discriminant: f64,
    t_min: f64,
    t_max: f64,
) -> Option<f64> {
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
