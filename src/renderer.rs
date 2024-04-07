use crate::camera::Camera;
use crate::writing::{FileWriter, ProgressWriter};
use crate::{Color, Hittable, Interval, Ray};

use std::io;

pub struct Renderer {
    samples_per_pixel: u32,
    max_ray_depth: u32,
}

impl Renderer {
    pub fn new(samples_per_pixel: u32, max_ray_depth: u32) -> Self {
        Self {
            samples_per_pixel,
            max_ray_depth,
        }
    }

    pub fn render(&self, world: &impl Hittable, camera: &Camera) -> io::Result<()> {
        let mut file_writer = FileWriter::new(camera.image_width(), camera.image_height())?;
        let progress_writer = ProgressWriter::new();

        for row in 0..camera.image_height() {
            for col in 0..camera.image_width() {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _sample in 1..=self.samples_per_pixel {
                    let ray = camera.get_ray(row, col);
                    pixel_color += Self::ray_color(&ray, self.max_ray_depth, world);
                }

                file_writer.write_pixel(pixel_color, self.samples_per_pixel)?;
            }

            progress_writer.print_progress(row, camera.image_height());
        }

        Ok(())
    }

    fn ray_color(ray: &Ray, depth: u32, world: &impl Hittable) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_rec) = world.hit(ray, Interval::new(0.001, f32::INFINITY)) {
            // At this point, the hit record should have a material, so we can unwrap
            let material = hit_rec.material.as_ref().unwrap();

            let color = if let Some(scatter) = material.scatter(ray, &hit_rec) {
                scatter.attenuation * Self::ray_color(&scatter.ray, depth - 1, world)
            } else {
                Color::new(0.0, 0.0, 0.0)
            };

            return color;
        }

        // Gradient background
        let white = Color::new(1.0, 1.0, 1.0);
        let blue = Color::new(0.5, 0.7, 1.0);
        let direction = ray.direction().normalized();
        let lerp_factor = 0.5 * (direction.y + 1.0);
        (1.0 - lerp_factor) * white + lerp_factor * blue
    }
}
