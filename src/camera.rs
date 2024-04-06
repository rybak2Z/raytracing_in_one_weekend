mod camera_builder;

pub use camera_builder::CameraBuilder;

use crate::random::random_range;
use crate::{Color, Hittable, Interval, Point3, Ray, Vec3, MAX_VALUE};

use std::io::{self, BufWriter, Write};
use std::time::Instant;

struct Orientation {
    #[allow(dead_code)]
    look_at: Point3,
    #[allow(dead_code)]
    view_up: Vec3,
}

struct Basis {
    up: Vec3,
    right: Vec3,
    back: Vec3,
}

struct Defocus {
    #[allow(dead_code)]
    focus_distance: f32,
    angle: f32,
    disk_u: Vec3,
    disk_v: Vec3,
}

impl Defocus {
    fn disk_sample(&self, position: Point3) -> Point3 {
        let offset = Vec3::random_in_unit_disk();
        position + (offset.x * self.disk_u) + (offset.y * self.disk_v)
    }
}

struct Image {
    width: u32,
    height: u32,
    #[allow(dead_code)]
    aspect_ratio: f32,
}

struct Viewport {
    #[allow(dead_code)]
    width: f32,
    #[allow(dead_code)]
    height: f32,
    pixel_top_left: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Viewport {
    pub fn pixel_sample(&self, row: u32, column: u32) -> Point3 {
        let offset_x = column as f32 * self.pixel_delta_u;
        let offset_y = row as f32 * self.pixel_delta_v;
        let pixel_center = self.pixel_top_left + offset_x + offset_y;
        pixel_center + self.random_pixel_square_sample()
    }

    fn random_pixel_square_sample(&self) -> Vec3 {
        let factor_x = random_range(-0.5, 0.5);
        let factor_y = random_range(-0.5, 0.5);
        (factor_x * self.pixel_delta_u) + (factor_y * self.pixel_delta_v)
    }
}

struct RenderOptions {
    samples_per_pixel: u32,
    max_depth: u32,
}

pub struct Camera {
    position: Point3,
    #[allow(dead_code)]
    vertical_fov: f32,
    #[allow(dead_code)]
    orientation: Orientation,
    #[allow(dead_code)]
    basis: Basis,
    defocus: Defocus,
    image: Image,
    viewport: Viewport,
    render_options: RenderOptions,
}

impl Camera {
    pub fn render(&self, world: &impl Hittable) -> io::Result<()> {
        let mut stdout = BufWriter::new(io::stdout());
        write!(
            stdout,
            "P3\n{} {}\n{}\n",
            self.image.width, self.image.height, MAX_VALUE
        )?;

        let time_start = Instant::now();

        for row in 0..self.image.height {
            for col in 0..self.image.width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _sample in 1..=self.render_options.samples_per_pixel {
                    let ray = self.get_ray(row, col);
                    pixel_color += Self::ray_color(&ray, self.render_options.max_depth, world);
                }

                write!(
                    stdout,
                    "{}",
                    pixel_color.pixel_format(self.render_options.samples_per_pixel)
                )?;
            }

            self.print_progress(row);
        }

        stdout.flush()?;
        self.print_finish(time_start);
        Ok(())
    }

    fn get_ray(&self, row: u32, column: u32) -> Ray {
        let pixel_sample = self.viewport.pixel_sample(row, column);

        let origin = match self.defocus.angle <= 0.0 {
            true => self.position,
            false => self.defocus.disk_sample(self.position),
        };
        let direction = pixel_sample - origin;

        Ray::new(origin, direction)
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

    fn print_progress(&self, row: u32) {
        let progress = row as f32 / (self.image.height - 1) as f32;
        let lines_remaining = self.image.height - (row + 1);
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
