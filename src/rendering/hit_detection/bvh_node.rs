#![allow(clippy::needless_borrow)]
#![allow(clippy::new_ret_no_self)]

use super::{HitRecord, Hittable, HittableEnum, HittableList, Ray, AABB};

use rand::{thread_rng, Rng};

use std::cmp::Ordering;
use std::sync::Arc;

type Comparator = fn(&Arc<HittableEnum>, &Arc<HittableEnum>) -> Ordering;

pub struct BvhNode {
    left: Arc<HittableEnum>,
    right: Arc<HittableEnum>,
    bbox: AABB,
}

impl BvhNode {
    pub fn new(list: &HittableList, time0: f64, time1: f64) -> Arc<HittableEnum> {
        Self::construct_tree(&list.objects[..], 0, list.objects.len(), time0, time1)
    }

    fn construct_tree(
        src_objects: &[Arc<HittableEnum>],
        from_obj: usize,
        up_to_obj: usize,
        time0: f64,
        time1: f64,
    ) -> Arc<HittableEnum> {
        let objects = &mut src_objects.to_owned()[from_obj..up_to_obj];
        let comparator = get_random_axis_comparator();

        let (left, right) = match objects.len() {
            1 => (objects[0].clone(), objects[0].clone()),
            2 => order_two(objects, comparator),
            _ => construct_sub_trees(objects, comparator, time0, time1),
        };

        let bbox = get_surrounding_box(&left, &right, time0, time1);
        Arc::new(BvhNode { left, right, bbox }.into())
    }
}

impl Hittable for BvhNode {
    fn hit(&self, ray_in: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(ray_in, t_min, t_max) {
            return None;
        }

        let hit_left = self.left.hit(ray_in, t_min, t_max);

        let t_max = match &hit_left {
            Some(hit_record) => hit_record.t,
            None => t_max,
        };
        let hit_right = self.right.hit(ray_in, t_min, t_max);

        if hit_right.is_some() {
            hit_right
        } else if hit_left.is_some() {
            hit_left
        } else {
            None
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}

fn get_random_axis_comparator() -> Comparator {
    let mut rng = thread_rng();
    let axis = rng.gen_range(0..3);
    match axis {
        0 => compare_box_x,
        1 => compare_box_y,
        2 => compare_box_z,
        _ => panic!("Invalid random axis in BvhNode constructor."),
    }
}

fn order_two(
    objects: &[Arc<HittableEnum>],
    comparator: Comparator,
) -> (Arc<HittableEnum>, Arc<HittableEnum>) {
    let first = &objects[0];
    let second = &objects[1];
    match comparator(first, second) {
        Ordering::Less | Ordering::Equal => (first.clone(), second.clone()),
        Ordering::Greater => (second.clone(), first.clone()),
    }
}

fn construct_sub_trees(
    objects: &mut [Arc<HittableEnum>],
    comparator: Comparator,
    time0: f64,
    time1: f64,
) -> (Arc<HittableEnum>, Arc<HittableEnum>) {
    objects.sort_by(comparator);
    let mid = objects.len() / 2;
    let left = BvhNode::construct_tree(&objects, 0, mid, time0, time1);
    let right = BvhNode::construct_tree(&objects, mid, objects.len(), time0, time1);

    (left, right)
}

fn get_surrounding_box(
    left: &Arc<HittableEnum>,
    right: &Arc<HittableEnum>,
    time0: f64,
    time1: f64,
) -> AABB {
    let left_box = left.bounding_box(time0, time1);
    let right_box = right.bounding_box(time0, time1);
    if left_box.is_none() || right_box.is_none() {
        panic!("No bounding box in BvhNode constructor.");
    }

    AABB::surrounding_box(left_box.unwrap(), right_box.unwrap())
}

fn compare_boxes(
    object_a: &Arc<HittableEnum>,
    object_b: &Arc<HittableEnum>,
    axis: usize,
) -> Ordering {
    let box_a = object_a.bounding_box(0.0, 0.0);
    let box_b = object_b.bounding_box(0.0, 0.0);

    if box_a.is_none() || box_b.is_none() {
        panic!("No bounding box in BvhNode constructor");
    }

    (box_a.unwrap().min().get(axis)).total_cmp(&box_b.unwrap().min().get(axis))
}

fn compare_box_x(object_a: &Arc<HittableEnum>, object_b: &Arc<HittableEnum>) -> Ordering {
    compare_boxes(object_a, object_b, 0)
}

fn compare_box_y(object_a: &Arc<HittableEnum>, object_b: &Arc<HittableEnum>) -> Ordering {
    compare_boxes(object_a, object_b, 1)
}

fn compare_box_z(object_a: &Arc<HittableEnum>, object_b: &Arc<HittableEnum>) -> Ordering {
    compare_boxes(object_a, object_b, 2)
}
