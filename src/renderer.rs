use crate::camera::Camera;
use crate::{Color, Hittable, Interval, Ray, MAX_VALUE};

use std::io::{self, BufWriter, Write};
use std::time::Instant;

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
        let mut stdout = BufWriter::new(io::stdout());
        write!(
            stdout,
            "P3\n{} {}\n{}\n",
            camera.image_width(),
            camera.image_height(),
            MAX_VALUE
        )?;

        let time_start = Instant::now();

        for row in 0..camera.image_height() {
            for col in 0..camera.image_width() {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _sample in 1..=self.samples_per_pixel {
                    let ray = camera.get_ray(row, col);
                    pixel_color += Self::ray_color(&ray, self.max_ray_depth, world);
                }

                write!(
                    stdout,
                    "{}",
                    pixel_color.pixel_format(self.samples_per_pixel)
                )?;
            }

            self.print_progress(row, camera);
        }

        stdout.flush()?;
        self.print_finish(time_start);
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

    fn print_progress(&self, row: u32, camera: &Camera) {
        let progress = row as f32 / (camera.image_height() - 1) as f32;
        let lines_remaining = camera.image_height() - (row + 1);
        let cleaning = "     "; // Needed if the current output line is shorter than the line that gets overwritten
        eprint!(
            "\rProgress: {:.2} % (scanlines remaining: {}){}",
            progress * 100.0,
            lines_remaining,
            cleaning
        );
    }

    fn print_finish(&self, time_start: Instant) {
        let duration = time_start.elapsed();
        let seconds = duration.as_secs();
        let minutes = seconds / 60;
        let rest_seconds = seconds % 60;
        eprintln!("\nFinished after {:02}m{:02}s", minutes, rest_seconds);
    }
}
