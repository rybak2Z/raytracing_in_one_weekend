use rand::prelude::*;
pub use std::rc::Rc;

mod hit_detection;
pub use hit_detection::*;

use crate::camera::Camera;
use crate::config::*;
use crate::ray::*;
use crate::vec3::*;
use crate::writing::*;

pub fn render(
    world: &HittableList,
    camera: Camera,
    writer: &mut BufWriter<StdoutLock>,
    writer_err: &mut BufWriter<StderrLock>,
) -> io::Result<()> {
    let mut rng = thread_rng();
    for row in (0..IMAGE_HEIGHT).rev() {
        write_progress_update(row, writer_err)?;
        for col in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::default();
            for _sample in 0..SAMPLES_PER_PIXEL {
                let u = (col as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (row as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += get_ray_color(&ray, world, MAX_DEPTH);
            }
            write_pixel(writer, pixel_color)?;
        }
    }

    Ok(())
}

fn get_ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::default();
    }

    if let Some(hit_record) = world.hit(ray, 0.0001, f64::INFINITY) {
        let (does_hit, scattered_ray, attenuation) = hit_record.material.scatter(ray, &hit_record);
        if does_hit {
            return attenuation * get_ray_color(&scattered_ray, world, depth - 1);
        }
        return Color::default();
    }

    let direction = ray.direction().normalized();
    let t = 0.5 * (direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
