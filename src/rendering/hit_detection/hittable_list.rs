use super::{Hittable, HitRecord, AABB};

use crate::rendering::Ray;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
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

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut temp_box = self.objects[0].bounding_box(time0, time1)?;

        for object in self.objects.iter() {
            if let Some(bbox) = object.bounding_box(time0, time1) {
                temp_box = AABB::surrounding_box(temp_box, bbox);
            } else {
                return None;
            }
        }

        Some(temp_box)
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
