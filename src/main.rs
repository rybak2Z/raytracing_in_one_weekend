use raytracing_in_one_weekend::camera::CameraBuilder;
use raytracing_in_one_weekend::hittable_list::HittableList;
use raytracing_in_one_weekend::sphere::Sphere;
use raytracing_in_one_weekend::{random, Dialectric, Material, Metal};
use raytracing_in_one_weekend::{Color, Lambertian, Point3, Vec3};

use std::io;
use std::rc::Rc;

fn main() -> io::Result<()> {
    random::initialize();

    let world = build_scene();

    let camera_builder = CameraBuilder {
        image_width: 100,
        aspect_ratio: 16.0 / 9.0,
        vertical_fov: 20.0,
        position: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        view_up: Vec3::new(0.0, 1.0, 0.0),
        focus_distance: 10.0,
        defocus_angle: 0.6,
        samples_per_pixel: 100,
        max_depth: 5,
    };

    let camera = camera_builder.finalize();

    camera.render(&world)
}

fn build_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for x in -11..11 {
        for z in -11..11 {
            let choose_material = random::random();
            let center = Point3::new(
                x as f32 + 0.9 * random::random(),
                0.2,
                z as f32 + 0.9 * random::random(),
            );

            let too_close = (center - Point3::new(4.0, 0.2, 0.0)).length() <= 0.9;
            if too_close {
                continue;
            }

            let material: Rc<dyn Material>;

            if choose_material < 0.8 {
                // Diffuse
                let albedo = Color::random() * Color::random();
                material = Rc::new(Lambertian::new(albedo));
            } else if choose_material < 0.95 {
                // Metal
                let albedo = Color::new(
                    random::random_range(0.5, 1.0),
                    random::random_range(0.5, 1.0),
                    random::random_range(0.5, 1.0),
                );
                let fuzz = random::random_range(0.0, 0.5);
                material = Rc::new(Metal::new(albedo, fuzz));
            } else {
                // Glass
                material = Rc::new(Dialectric::new(1.5));
            }

            let sphere = Sphere::new(center, 0.2, material);
            world.add(Rc::new(sphere));
        }
    }

    let material_1 = Rc::new(Dialectric::new(1.5));
    let position_1 = Point3::new(0.0, 1.0, 0.0);
    let sphere_1 = Sphere::new(position_1, 1.0, material_1);
    world.add(Rc::new(sphere_1));

    let material_2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let position_2 = Point3::new(-4.0, 1.0, 0.0);
    let sphere_2 = Sphere::new(position_2, 1.0, material_2);
    world.add(Rc::new(sphere_2));

    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    let position_3 = Point3::new(4.0, 1.0, 0.0);
    let sphere_3 = Sphere::new(position_3, 1.0, material_3);
    world.add(Rc::new(sphere_3));

    world
}
