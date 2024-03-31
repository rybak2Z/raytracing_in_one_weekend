use raytracing_in_one_weekend::camera::Camera;
use raytracing_in_one_weekend::hittable_list::HittableList;
use raytracing_in_one_weekend::random;
use raytracing_in_one_weekend::sphere::Sphere;
use raytracing_in_one_weekend::{Color, Lambertian, Material, Point3};

use std::io;
use std::rc::Rc;

fn main() -> io::Result<()> {
    random::initialize();

    let mut world = HittableList::default();

    let color = Color::new(0.5, 0.5, 0.5);
    let lambertian: Rc<dyn Material> = Rc::new(Lambertian::new(color));

    let sphere1_pos = Point3::new(0.0, 0.0, -1.0);
    let sphere1_radius = 0.5;
    let sphere1 = Sphere::new(sphere1_pos, sphere1_radius, Rc::clone(&lambertian));

    let sphere2_pos = Point3::new(0.0, -100.5, -1.0);
    let sphere2_radius = 100.0;
    let sphere2 = Sphere::new(sphere2_pos, sphere2_radius, Rc::clone(&lambertian));

    world.add(Rc::new(sphere1));
    world.add(Rc::new(sphere2));

    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let max_depth = 10;
    let camera = Camera::new(image_width, aspect_ratio, samples_per_pixel, max_depth);

    camera.render(&world)
}
