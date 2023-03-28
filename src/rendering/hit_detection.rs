use super::{Color, Point3, Ray, Rc, Vec3};
use crate::material::{Lambertian, Material};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Rc<dyn Material>,
    pub t: f64,
    pub on_front_face: bool,
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

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.on_front_face = Vec3::dot(ray.direction(), outward_normal) < 0.0;
        self.normal = match self.on_front_face {
            true => outward_normal,
            false => -outward_normal,
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_anything = false;
        let mut closest = HitRecord {
            t: t_max,
            ..Default::default()
        };

        for object in self.objects.iter() {
            let hit_record = object.hit(ray, t_min, t_max);
            if let Some(record) = hit_record {
                if record.t < closest.t {
                    hit_anything = true;
                    closest = record;
                }
            }
        }

        match hit_anything {
            true => Some(closest),
            false => None,
        }
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
