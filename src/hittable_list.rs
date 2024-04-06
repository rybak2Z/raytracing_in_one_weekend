use crate::{HitRecord, Hittable, Interval};

use std::rc::Rc;

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl HittableList {
    pub fn new(objects: Vec<Rc<dyn Hittable>>) -> Self {
        Self { objects }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
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
