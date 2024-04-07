use crate::{HitRecord, Hittable, Interval, SharedHittable};

use std::sync::Arc;

#[derive(Default, Clone)]
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable + Send + Sync>>,
}

impl HittableList {
    pub fn new(objects: Vec<SharedHittable>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: SharedHittable) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit_mutating(&self, ray: &crate::Ray, allowed_t: Interval, hit_rec: &mut HitRecord) -> bool {
        let mut current_rec = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_t = allowed_t.max;

        for object in &self.objects {
            if object.hit_mutating(
                ray,
                Interval::new(allowed_t.min, closest_t),
                &mut current_rec,
            ) {
                hit_anything = true;
                closest_t = current_rec.t;
            }
        }

        if hit_anything {
            *hit_rec = current_rec;
        }

        hit_anything
    }
}
