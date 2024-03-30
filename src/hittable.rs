use crate::{Interval, Point3, Ray, Vec3};

#[derive(Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new(point: Point3, normal: Vec3, t: f32) -> HitRecord {
        HitRecord {
            point,
            normal,
            t,
            front_face: false,
        }
    }

    /// The parameter 'outward_normal' is expected to have unit length.
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit_mutating(&self, ray: &Ray, allowed_t: Interval, hit_rec: &mut HitRecord) -> bool;

    fn hit(&self, ray: &Ray, allowed_t: Interval) -> Option<HitRecord> {
        let mut hit_record = HitRecord::default();
        match self.hit_mutating(ray, allowed_t, &mut hit_record) {
            true => Some(hit_record),
            false => None,
        }
    }
}
