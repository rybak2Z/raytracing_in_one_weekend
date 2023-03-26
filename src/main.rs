use rand::prelude::*;

use raytracing_in_one_weekend::rendering::*;
use raytracing_in_one_weekend::vec3::{Color, Point3};
use raytracing_in_one_weekend::writing::*;

fn main() -> io::Result<()> {
    let (mut writer, mut writer_err) = get_writers();
    write_meta_data(&mut writer)?;
    let world = build_random_world();
    render(&world, &mut writer, &mut writer_err)?;
    finish_writers(&mut writer, &mut writer_err)?;
    Ok(())
}

fn _build_world() -> HittableList {
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dialectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let sphere_ground = Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    );
    let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, material_center.clone());
    let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, material_left.clone());
    let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, material_right.clone());

    let mut world = HittableList::new();
    world.add(Rc::new(sphere_ground));
    world.add(Rc::new(sphere_center));
    world.add(Rc::new(sphere_left));
    world.add(Rc::new(sphere_right));

    world
}

fn build_random_world() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Rc::new(Lambertian::new(Color::new(0.62, 0.76, 0.76)));
    let ground_sphere = Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material.clone(),
    );
    world.add(Rc::new(ground_sphere));

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.2,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material>;

                if choose_material < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Rc::new(Lambertian::new(albedo));
                } else if choose_material < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzziness = rng.gen_range(0.0..0.5);
                    sphere_material = Rc::new(Metal::new(albedo, fuzziness));
                } else {
                    // Glass
                    sphere_material = Rc::new(Dialectric::new(1.5));
                }

                let radius = rng.gen_range(0.15..0.25);
                world.add(Rc::new(Sphere::new(
                    center,
                    radius,
                    sphere_material.clone(),
                )));

                if choose_material > 0.98 {
                    // Make glass sphere hollow
                    let inner_sphere = Sphere::new(center, -radius * 0.85, sphere_material.clone());
                    world.add(Rc::new(inner_sphere));
                }
            }
        }
    }

    let material_1 = Rc::new(Dialectric::new(1.5));
    let material_2 = Rc::new(Lambertian::new(Color::new(0.62, 0.92, 0.47)));
    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    world
}
