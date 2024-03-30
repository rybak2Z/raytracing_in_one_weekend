use raytracing_in_one_weekend::camera::Camera;
use raytracing_in_one_weekend::hittable_list::HittableList;
use raytracing_in_one_weekend::random;
use raytracing_in_one_weekend::sphere::Sphere;
use raytracing_in_one_weekend::Point3;

use std::io;
use std::rc::Rc;

fn main() -> io::Result<()> {
    random::initialize();

    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let samples_per_pixel = 100;
    let camera = Camera::new(image_width, aspect_ratio, samples_per_pixel);

    camera.render(&world)
}
