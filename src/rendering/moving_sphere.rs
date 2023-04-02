use super::{
    sphere::find_smallest_valid_solution, HitRecord, Hittable, Material, Point3, Ray, Vec3,
};

#[derive(Clone)]
pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Box<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        center0: Point3,
        center1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Box<dyn Material>,
    ) -> MovingSphere {
        MovingSphere {
            center0,
            center1,
            time0,
            time1,
            radius,
            material,
        }
    }

    pub fn center(&self, time: f64) -> Point3 {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center_to_origin = ray.origin() - self.center(ray.time());

        // Quadratic equation
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(center_to_origin, ray.direction());
        let c = center_to_origin.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        let solution = find_smallest_valid_solution(a, half_b, discriminant, t_min, t_max)?;

        let mut record = HitRecord::default();
        record.t = solution;
        record.point = ray.at(record.t);
        let outward_normal = (record.point - self.center(ray.time())) / self.radius;
        record.set_face_normal(ray, outward_normal);
        record.material = self.material.clone();

        Some(record)
    }
}
