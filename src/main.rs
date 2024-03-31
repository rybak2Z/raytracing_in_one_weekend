use raytracing_in_one_weekend::camera::Camera;
use raytracing_in_one_weekend::hittable_list::HittableList;
use raytracing_in_one_weekend::material::Metal;
use raytracing_in_one_weekend::random;
use raytracing_in_one_weekend::sphere::Sphere;
use raytracing_in_one_weekend::{Color, Lambertian, Point3};

use std::io;
use std::rc::Rc;

fn main() -> io::Result<()> {
    random::initialize();

    let mut world = HittableList::default();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    let pos_ground = Point3::new(0.0, -100.5, -1.0);
    let pos_center = Point3::new(0.0, 0.0, -1.0);
    let pos_left = Point3::new(-1.0, 0.0, -1.0);
    let pos_right = Point3::new(1.0, 0.0, -1.0);

    let sphere_ground = Sphere::new(pos_ground, 100.0, material_ground);
    let sphere_center = Sphere::new(pos_center, 0.5, material_center);
    let sphere_left = Sphere::new(pos_left, 0.5, material_left);
    let sphere_right = Sphere::new(pos_right, 0.5, material_right);

    world.add(Rc::new(sphere_ground));
    world.add(Rc::new(sphere_center));
    world.add(Rc::new(sphere_left));
    world.add(Rc::new(sphere_right));

    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let max_depth = 10;
    let camera = Camera::new(image_width, aspect_ratio, samples_per_pixel, max_depth);

    camera.render(&world)
}
