use raytracing_in_one_weekend::camera::Camera;
use raytracing_in_one_weekend::hittable_list::HittableList;
use raytracing_in_one_weekend::sphere::Sphere;
use raytracing_in_one_weekend::{random, Dialectric, Metal};
use raytracing_in_one_weekend::{Color, Lambertian, Point3, Vec3};

use std::io;
use std::rc::Rc;

fn main() -> io::Result<()> {
    random::initialize();

    let mut world = HittableList::default();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dialectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let pos_ground = Point3::new(0.0, -100.5, -1.0);
    let pos_center = Point3::new(0.0, 0.0, -1.0);
    let pos_left = Point3::new(-1.0, 0.0, -1.0);
    let pos_right = Point3::new(1.0, 0.0, -1.0);

    let sphere_ground = Sphere::new(pos_ground, 100.0, material_ground);
    let sphere_center = Sphere::new(pos_center, 0.5, material_center);
    let sphere_left = Sphere::new(pos_left, 0.5, material_left.clone());
    let sphere_left_inner = Sphere::new(pos_left, -0.4, material_left);
    let sphere_right = Sphere::new(pos_right, 0.5, material_right);

    world.add(Rc::new(sphere_ground));
    world.add(Rc::new(sphere_center));
    world.add(Rc::new(sphere_left));
    world.add(Rc::new(sphere_left_inner));
    world.add(Rc::new(sphere_right));

    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let vertical_fov = 20.0;
    let look_from = Point3::new(-2.0, 2.0, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let view_up = Vec3::new(0.0, 1.0, 0.0);
    let focus_distance = 3.4;
    let defocus_angle = 10.0;
    let samples_per_pixel = 100;
    let max_depth = 10;

    let camera = Camera::new(
        image_width,
        aspect_ratio,
        vertical_fov,
        look_from,
        look_at,
        view_up,
        focus_distance,
        defocus_angle,
        samples_per_pixel,
        max_depth,
    );

    camera.render(&world)
}
