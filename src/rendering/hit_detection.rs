use super::{
    material::{Lambertian, Material},
    Color, Point3, Ray, Vec3, AABB,
};

pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub material: Box<dyn Material>,
    pub t: f64,
    pub on_front_face: bool,
}

impl HitRecord {
    fn new() -> HitRecord {
        HitRecord {
            point: Point3::default(),
            normal: Vec3::default(),
            material: Box::new(Lambertian::new(Color::default())),
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

pub trait Hittable: CloneHittable + Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB>;
}

// from https://users.rust-lang.org/t/solved-is-it-possible-to-clone-a-boxed-trait-object/1714/7
pub trait CloneHittable {
    fn clone_hittable(&self) -> Box<dyn Hittable>;
}

impl<T: Hittable + Clone + 'static> CloneHittable for T {
    fn clone_hittable(&self) -> Box<dyn Hittable> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Hittable> {
    fn clone(&self) -> Self {
        self.clone_hittable()
    }
}

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
