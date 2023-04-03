#![allow(clippy::upper_case_acronyms)]

use crate::rendering::{Point3, Ray};

#[derive(Clone)]
struct AABB {
    minimum: Point3,
    maximum: Point3,
}

impl AABB {
    pub fn new(a: Point3, b: Point3) -> AABB {
        AABB {
            minimum: a,
            maximum: b,
        }
    }

    fn min(&self) -> Point3 {
        self.minimum
    }

    fn max(&self) -> Point3 {
        self.maximum
    }
}

impl AABB {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let dimensions = 0..3;
        for d in dimensions {
            let inv_direction = 1.0 / ray.direction().get(d);
            let mut t0 = (self.min().get(d) - ray.origin().get(d)) * inv_direction;
            let mut t1 = (self.max().get(d) - ray.origin().get(d)) * inv_direction;

            if inv_direction < 0.0 {
                (t0, t1) = (t1, t0);
            }

            let t_min = t_min.max(t0);
            let t_max = t_max.min(t1);

            if t_max <= t_min {
                return false;
            }
        }

        true
    }
}
