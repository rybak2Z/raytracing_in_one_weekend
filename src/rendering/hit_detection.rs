mod aabb;
mod bvh_node;
mod hittable_list;

pub use aabb::AABB;
pub use bvh_node::BvhNode;
pub use hittable_list::HittableList;

use super::{Color, Lambertian, MaterialEnum, MovingSphere, Point3, Ray, Sphere, Vec3};

use enum_dispatch::enum_dispatch;

use std::sync::Arc;

#[enum_dispatch(Hittable)]
pub enum HittableEnum {
    Sphere,
    MovingSphere,
    HittableList,
    BvhNode,
}

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Arc<MaterialEnum>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub on_front_face: bool,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        t: f64,
        u: f64,
        v: f64,
        outward_normal: Vec3,
        material: Arc<MaterialEnum>,
    ) -> HitRecord {
        let on_front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        let normal = match on_front_face {
            true => outward_normal,
            false => -outward_normal,
        };

        HitRecord {
            point: ray.at(t),
            normal,
            material,
            t,
            u,
            v,
            on_front_face,
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new(
            &Ray::default(),
            0.0,
            0.0,
            0.0,
            Vec3::default(),
            Arc::new(Lambertian::new(Color::default()).into()),
        )
    }
}

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}
