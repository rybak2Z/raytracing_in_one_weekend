use raytracing_in_one_weekend::rendering::*;
use raytracing_in_one_weekend::vec3::{Color, Point3};
use raytracing_in_one_weekend::writing::*;

fn main() -> io::Result<()> {
    let (mut writer, mut writer_err) = get_writers();
    write_meta_data(&mut writer)?;
    let world = build_world();
    render(&world, &mut writer, &mut writer_err)?;
    finish_writers(&mut writer, &mut writer_err)?;
    Ok(())
}

fn build_world() -> HittableList {
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

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
