use raytracing_in_one_weekend::rendering::*;
use raytracing_in_one_weekend::vec3::Point3;
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
    let mut world = HittableList::new();
    let sphere_1 = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5);
    let sphere_2 = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0);
    world.add(Rc::new(sphere_1));
    world.add(Rc::new(sphere_2));

    world
}
