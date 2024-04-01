use crate::random::random_range;
use crate::{Color, Hittable, Interval, Point3, Ray, Vec3, MAX_VALUE};

use std::io::{self, BufWriter, Write};
use std::time::Instant;

pub struct Camera {
    image_width: u32,
    image_height: u32,
    #[allow(dead_code)]
    aspect_ratio: f32,
    #[allow(dead_code)]
    vertical_fov: f32,
    samples_per_pixel: u32,
    max_depth: u32,
    position: Point3,
    pixel_top_left: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(
        image_width: u32,
        aspect_ratio: f32,
        vertical_fov: f32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Camera {
        let image_height = (image_width as f32 / aspect_ratio).round() as u32;
        let image_height = image_height.max(1);

        let position = Point3::zero();

        // Determine viewport dimensions
        let focal_length = 1.0;
        let theta = vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
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
            vertical_fov,
            samples_per_pixel,
            max_depth,
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
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _sample in 1..=self.samples_per_pixel {
                    let ray = self.get_ray(row, col);
                    pixel_color += Camera::ray_color(&ray, self.max_depth, world);
                }

                write!(
                    stdout,
                    "{}",
                    pixel_color.pixel_format(self.samples_per_pixel)
                )?;
            }

            self.print_progress(row);
        }

        stdout.flush()?;
        self.print_finish(time_start);
        Ok(())
    }

    fn get_ray(&self, row: u32, column: u32) -> Ray {
        let offset_x = column as f32 * self.pixel_delta_u;
        let offset_y = row as f32 * self.pixel_delta_v;
        let pixel_center = self.pixel_top_left + offset_x + offset_y;
        let pixel_sample = pixel_center + self.pixel_square_sample();

        let origin = self.position;
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
    }

    fn pixel_square_sample(&self) -> Vec3 {
        let factor_x = random_range(-0.5, 0.5);
        let factor_y = random_range(-0.5, 0.5);
        (factor_x * self.pixel_delta_u) + (factor_y * self.pixel_delta_v)
    }

    fn ray_color(ray: &Ray, depth: u32, world: &impl Hittable) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_rec) = world.hit(ray, Interval::new(0.001, f32::INFINITY)) {
            // At this point, the hit record should have a material, so we can unwrap
            let material = hit_rec.material.as_ref().unwrap();

            let color = if let Some(scatter) = material.scatter(ray, &hit_rec) {
                scatter.attenuation * Camera::ray_color(&scatter.ray, depth - 1, world)
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
