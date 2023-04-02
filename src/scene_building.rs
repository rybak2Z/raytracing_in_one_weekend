use crate::rendering::{
    camera::{CameraConfiguration, Camera}, hit_detection::HittableList, material::*, sphere::Sphere, vec3::*,
};

use rand::prelude::*;

pub fn build_scene() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let ground_material = Box::new(Lambertian::new(Color::new(0.62, 0.76, 0.76)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(Box::new(ground_sphere));

    let mut rng = thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let choose_material: f64 = rng.gen();
            let center = Point3::new(
                a as f64 + 0.9 * rng.gen::<f64>(),
                0.0,
                b as f64 + 0.9 * rng.gen::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Box<dyn Material>;

                if choose_material < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Box::new(Lambertian::new(albedo));
                } else if choose_material < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzziness = rng.gen_range(0.0..0.5);
                    sphere_material = Box::new(Metal::new(albedo, fuzziness));
                } else {
                    // Glass
                    sphere_material = Box::new(Dialectric::new(1.5));
                }

                let radius = rng.gen_range(0.15..0.25);
                let center = center + Vec3::new(0.0, radius, 0.0);
                world.add(Box::new(Sphere::new(
                    center,
                    radius,
                    sphere_material.clone(),
                )));

                if choose_material > 0.98 {
                    // Make glass sphere hollow
                    let inner_sphere = Sphere::new(center, -radius * 0.85, sphere_material.clone());
                    world.add(Box::new(inner_sphere));
                }
            }
        }
    }

    let material_1 = Box::new(Dialectric::new(1.5));
    let material_2 = Box::new(Lambertian::new(Color::new(0.62, 0.92, 0.47)));
    let material_3 = Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3,
    )));

    let cam_config = CameraConfiguration {
        look_from: Point3::new(13.0, 2.0, 3.0),
        look_at: Point3::new(0.0, 0.0, 0.0),
        view_up: Vec3::new(0.0, 1.0, 0.0),
        vertical_fov: 20.0,
        aperture: 0.1,
        focus_distance: Some(10.0),
        focal_length: 1.0,
        start_time: None,
        end_time: None,
    };

    (world, Camera::new(cam_config))
}
