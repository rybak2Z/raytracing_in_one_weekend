use crate::{Color, Hittable, Interval, Point3, Ray, Vec3, MAX_VALUE};

use std::io::{self, BufWriter, Write};
use std::time::Instant;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    #[allow(dead_code)]
    aspect_ratio: f32,
    position: Point3,
    pixel_top_left: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(image_width: u32, aspect_ratio: f32) -> Camera {
        let image_height = (image_width as f32 / aspect_ratio).round() as u32;
        let image_height = image_height.max(1);

        let position = Point3::zero();

        // Determine viewport dimensions
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let actual_aspect_ratio = image_width as f32 / image_height as f32;
        let viewport_width = viewport_height * actual_aspect_ratio;

        // Vectors across the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f32;
        let pixel_delta_v = viewport_v / image_height as f32;

        // Calculate position of top left pixel
        let viewport_center = position - Vec3::new(0.0, 0.0, focal_length);
        let viewport_top_left = viewport_center - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_top_left = viewport_top_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

        Camera {
            image_width,
            image_height,
            aspect_ratio,
            position,
            pixel_top_left,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: &impl Hittable) -> io::Result<()> {
        let mut stdout = BufWriter::new(io::stdout());
        write!(
            stdout,
            "P3\n{} {}\n{}\n",
            self.image_width, self.image_height, MAX_VALUE
        )?;

        let time_start = Instant::now();

        for row in 0..self.image_height {
            for col in 0..self.image_width {
                let horizontal_offset = self.pixel_delta_u * col as f32;
                let vertical_offset = self.pixel_delta_v * row as f32;
                let pixel = self.pixel_top_left + horizontal_offset + vertical_offset;

                let ray_direction = pixel - self.position;
                let ray = Ray::new(self.position, ray_direction);
                let pixel_color = self.ray_color(&ray, world);

                write!(stdout, "{}", pixel_color.pixel_format())?;
            }

            writeln!(stdout)?;
            self.print_progress(row);
        }

        stdout.flush()?;
        self.print_finish(time_start);
        Ok(())
    }

    fn ray_color(&self, ray: &Ray, world: &impl Hittable) -> Color {
        if let Some(hit_rec) = world.hit(ray, Interval::new(0.0, f32::INFINITY)) {
            let Vec3 { x, y, z } = hit_rec.normal;
            return 0.5 * Color::new(1.0 + x, 1.0 + y, 1.0 + z);
        }

        // Gradient background
        let white = Color::new(1.0, 1.0, 1.0);
        let blue = Color::new(0.5, 0.7, 1.0);
        let direction = ray.direction().normalized();
        let lerp_factor = 0.5 * (direction.y + 1.0);
        (1.0 - lerp_factor) * white + lerp_factor * blue
    }

    fn print_progress(&self, row: u32) {
        let progress = row as f32 / (self.image_height - 1) as f32;
        let lines_remaining = self.image_height - (row + 1);
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
