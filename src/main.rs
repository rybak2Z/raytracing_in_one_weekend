use raytracing_in_one_weekend::camera::Camera;
use raytracing_in_one_weekend::hittable_list::HittableList;
use raytracing_in_one_weekend::random;
use raytracing_in_one_weekend::sphere::Sphere;
use raytracing_in_one_weekend::{Color, Lambertian, Point3};

use std::io;
use std::rc::Rc;

fn main() -> io::Result<()> {
    random::initialize();

    let mut world = HittableList::default();

    let material_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    let r = (std::f32::consts::PI / 4.0).cos();
    let pos_left = Point3::new(-r, 0.0, -1.0);
    let pos_right = Point3::new(r, 0.0, -1.0);

    let sphere_left = Sphere::new(pos_left, r, material_left);
    let sphere_right = Sphere::new(pos_right, r, material_right);

    world.add(Rc::new(sphere_left));
    world.add(Rc::new(sphere_right));

    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let vertical_fov = 90.0;
    let samples_per_pixel = 100;
    let max_depth = 10;
    let camera = Camera::new(
        image_width,
        aspect_ratio,
        vertical_fov,
        samples_per_pixel,
        max_depth,
    );

    camera.render(&world)
}
