use raytracing_in_one_weekend::color::Color;
use raytracing_in_one_weekend::{Point3, Ray, Vec3, MAX_VALUE};

use std::io::{self, BufWriter, Write};
use std::time::Instant;

fn main() -> io::Result<()> {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;

    let image_height = (image_width as f32 / aspect_ratio).round() as u32;
    let image_height = image_height.max(1);

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let actual_aspect_ratio = image_width as f32 / image_height as f32;
    let viewport_width = viewport_height * actual_aspect_ratio;
    let camera_pos = Point3::zero();

    // Vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Calculate position of top left pixel
    let viewport_center = camera_pos - Vec3::new(0.0, 0.0, focal_length);
    let viewport_top_left = viewport_center - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_top_left = viewport_top_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

    // Render

    let mut stdout = BufWriter::new(io::stdout());
    write!(stdout, "P3\n{image_width} {image_height}\n{MAX_VALUE}\n",)?;

    let time_start = Instant::now();

    for row in 0..image_height {
        for col in 0..image_width {
            let horizontal_offset = pixel_delta_u * col as f32;
            let vertical_offset = pixel_delta_v * row as f32;
            let pixel = pixel_top_left + horizontal_offset + vertical_offset;

            let ray_direction = pixel - camera_pos;
            let ray = Ray::new(camera_pos, ray_direction);
            let pixel_color = ray_color(&ray);

            write!(stdout, "{}", pixel_color.pixel_format())?;
        }

        writeln!(stdout)?;
        print_progress(row, image_height);
    }

    stdout.flush()?;
    print_finish(time_start);
    Ok(())
}

fn ray_color(ray: &Ray) -> Color {
    let sphere_center = Point3::new(0.0, 0.0, -1.0);
    let sphere_radius = 0.5;
    let t = hit_sphere(sphere_center, sphere_radius, ray);

    if t > 0.0 {
        let normal = (ray.at(t) - sphere_center).normalized();
        return 0.5 * Color::new(1.0 + normal.x, 1.0 + normal.y, 1.0 + normal.z);
    }

    // Gradient background
    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);
    let direction = ray.direction().normalized();
    let lerp_factor = 0.5 * (direction.y + 1.0);
    (1.0 - lerp_factor) * white + lerp_factor * blue
}

fn hit_sphere(center: Point3, radius: f32, ray: &Ray) -> f32 {
    // Following the equation from chapter 5.2
    let oc = ray.origin() - center;
    let a = Vec3::dot(ray.direction(), ray.direction());
    let b = 2.0 * Vec3::dot(oc, ray.direction());
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn print_progress(row: u32, image_height: u32) {
    let progress = row as f32 / (image_height - 1) as f32;
    let lines_remaining = image_height - (row + 1);
    let cleaning = "     "; // Needed if the current output line is shorter than the line that gets overwritten
    eprint!(
        "\rProgress: {:.2} % (scanlines remaining: {}){}",
        progress * 100.0,
        lines_remaining,
        cleaning
    );
}

fn print_finish(time_start: Instant) {
    let duration = time_start.elapsed();
    let seconds = duration.as_secs();
    let minutes = seconds / 60;
    let rest_seconds = seconds % 60;
    eprintln!("\nFinished after {:02}m{:02}s", minutes, rest_seconds);
}
