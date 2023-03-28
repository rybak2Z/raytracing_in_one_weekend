use rand::prelude::*;
pub use std::rc::Rc;

mod hit_detection;
pub use hit_detection::*;

use crate::camera::Camera;
use crate::config::*;
use crate::ray::*;
use crate::vec3::*;
use crate::writing::*;

struct RenderingTools<'a> {
    world: &'a HittableList,
    camera: &'a Camera,
    rng: &'a mut ThreadRng,
}

pub fn render(
    world: &HittableList,
    camera: Camera,
    writer: &mut Writer,
    writer_err: &mut WriterErr,
) -> io::Result<()> {
    let mut rng = thread_rng();
    let mut render_tools = RenderingTools {
        world,
        camera: &camera,
        rng: &mut rng,
    };

    for row in (0..IMAGE_HEIGHT).rev() {
        write_progress_update(row, writer_err)?;

        for col in 0..IMAGE_WIDTH {
            let accumulated_color = accumulate_pixel_color_samples(row, col, &mut render_tools);
            let mut pixel_color = accumulated_color / SAMPLES_PER_PIXEL as f64;
            correct_gamma(&mut pixel_color);
            write_pixel(writer, pixel_color)?;
        }
    }

    Ok(())
}

fn accumulate_pixel_color_samples(row: u32, col: u32, render_tools: &mut RenderingTools) -> Color {
    let mut accumulated_color = Color::default();
    for _sample in 0..SAMPLES_PER_PIXEL {
        accumulated_color += calculate_pixel_color(row, col, render_tools);
    }

    accumulated_color
}

fn calculate_pixel_color(row: u32, col: u32, render_tools: &mut RenderingTools) -> Color {
    let (u, v) = get_uv(row, col, render_tools.rng);
    let ray = render_tools.camera.get_ray(u, v);
    get_ray_color(&ray, render_tools.world, MAX_DEPTH)
}

fn get_uv(row: u32, col: u32, rng: &mut ThreadRng) -> (f64, f64) {
    let u = (col as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
    let v = (row as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
    (u, v)
}

fn get_ray_color(ray: &Ray, world: &HittableList, depth: i32) -> Color {
    if depth == 0 {
        return Color::default();
    }

    if let Some(hit_record) = world.hit(ray, 0.0001, f64::INFINITY) {
        if let Some(scatter) = hit_record.material.scatter(ray, &hit_record) {
            return scatter.attenuation * get_ray_color(&scatter.ray, world, depth - 1);
        }
        return Color::default();
    }

    get_sky_color(ray)
}

fn get_sky_color(ray: &Ray) -> Color {
    let direction = ray.direction().normalized();
    let blend_factor = 0.5 * (direction.y() + 1.0);

    let white_part = (1.0 - blend_factor) * Color::new(1.0, 1.0, 1.0);
    let blue_part = blend_factor * Color::new(0.5, 0.7, 1.0);

    white_part + blue_part
}

fn correct_gamma(color: &mut Color) {
    color.set_r(color.r().sqrt());
    color.set_g(color.g().sqrt());
    color.set_b(color.b().sqrt());
}
