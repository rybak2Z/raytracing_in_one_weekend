use crate::rendering::{
    camera::{Camera, CameraConfiguration},
    hit_detection::HittableList,
    material::*,
    sphere::Sphere,
    texture::*,
    vec3::*,
};

use rand::prelude::*;

use std::sync::Arc;

pub fn build_scene() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let checker_texture = Arc::new(TextureEnum::from(CheckerBoard::from_colors(
        Color::new(0.42, 0.56, 0.56),
        Color::new(0.9, 0.9, 0.9),
    )));
    let ground_material = Arc::new(MaterialEnum::from(Lambertian::from_texture(
        checker_texture,
    )));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_material);
    world.add(Arc::new(ground_sphere.into()));

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
                let sphere_material: Arc<MaterialEnum>;

                if choose_material < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo).into());
                } else if choose_material < 0.95 {
                    // Metal
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzziness = rng.gen_range(0.0..0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzziness).into());
                } else {
                    // Glass
                    sphere_material = Arc::new(Dialectric::new(1.5).into());
                }

                let radius = rng.gen_range(0.15..0.25);
                let center = center + Vec3::new(0.0, radius, 0.0);
                world.add(Arc::new(
                    Sphere::new(center, radius, sphere_material.clone()).into(),
                ));

                if choose_material > 0.98 {
                    // Make glass sphere hollow
                    let inner_sphere = Sphere::new(center, -radius * 0.85, sphere_material.clone());
                    world.add(Arc::new(inner_sphere.into()));
                }
            }
        }
    }

    let material_1 = Arc::new(MaterialEnum::from(Dialectric::new(1.5)));
    let material_2 = Arc::new(MaterialEnum::from(Lambertian::new(Color::new(
        0.62, 0.92, 0.47,
    ))));
    let material_3 = Arc::new(MaterialEnum::from(Metal::new(
        Color::new(0.7, 0.6, 0.5),
        0.0,
    )));
    world.add(Arc::new(
        Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material_1).into(),
    ));
    world.add(Arc::new(
        Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material_2).into(),
    ));
    world.add(Arc::new(
        Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material_3).into(),
    ));

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
