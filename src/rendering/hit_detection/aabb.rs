#![allow(clippy::upper_case_acronyms)]

use crate::rendering::{Point3, Ray};

#[derive(Copy, Clone)]
pub struct AABB {
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

    pub fn min(&self) -> Point3 {
        self.minimum
    }

    pub fn max(&self) -> Point3 {
        self.maximum
    }
}

impl AABB {
    pub fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> bool {
        let dimensions = 0..3;
        for d in dimensions {
            let (t0, t1) = self.get_intersect_interval(ray, d);
            if !do_intervals_overlap(t_min, t_max, t0, t1) {
                return false;
            }
        }

        true
    }

    fn get_intersect_interval(&self, ray: &Ray, dimension: usize) -> (f64, f64) {
        let d = dimension;
        let inv_direction = 1.0 / ray.direction().get(d);
        let mut t0 = (self.min().get(d) - ray.origin().get(d)) * inv_direction;
        let mut t1 = (self.max().get(d) - ray.origin().get(d)) * inv_direction;

        if inv_direction < 0.0 {
            (t0, t1) = (t1, t0);
        }

        (t0, t1)
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let smallest = Point3::new(
            (box0.min().x()).min(box1.min().x()),
            (box0.min().y()).min(box1.min().y()),
            (box0.min().z()).min(box1.min().z()),
        );

        let biggest = Point3::new(
            (box0.max().x()).max(box1.max().x()),
            (box0.max().y()).max(box1.max().y()),
            (box0.max().z()).max(box1.max().z()),
        );

        AABB::new(smallest, biggest)
    }
}

impl std::default::Default for AABB {
    fn default() -> Self {
        AABB::new(Point3::default(), Point3::default())
    }
}

fn do_intervals_overlap(a_start: f64, a_end: f64, b_start: f64, b_end: f64) -> bool {
    let c_start = a_start.max(b_start);
    let c_end = a_end.min(b_end);

    c_start < c_end
}
