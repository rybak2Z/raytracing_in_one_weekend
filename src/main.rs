use raytracing_in_one_weekend::ray::Ray;
use raytracing_in_one_weekend::vec3::{Color, Point3, Vec3};
use raytracing_in_one_weekend::writing::*;

// Image
const ASCPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASCPECT_RATIO) as u32;

// Camera
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = ASCPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f64 = 1.0;
const ORIGIN: Point3 = Point3::new(0.0, 0.0, 0.0);
const HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
const VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);

// File format
const FILE_TYPE: &str = "P3";
const MAX_COLOR: u32 = 255;

fn main() -> io::Result<()> {
    // Should be a constant but unable to evaluate at compile-time
    let lower_left_corner =
        ORIGIN - HORIZONTAL / 2.0 - VERTICAL / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    let (mut writer, mut writer_err) = get_writers();
    write!(
        writer,
        "{FILE_TYPE}\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n{MAX_COLOR}\n"
    )?;

    render(&mut writer, &mut writer_err, lower_left_corner)?;

    finish_writers(&mut writer, &mut writer_err)?;

    Ok(())
}

fn render(
    writer: &mut BufWriter<StdoutLock>,
    writer_err: &mut BufWriter<StderrLock>,
    lower_left: Point3,
) -> io::Result<()> {
    for row in (0..IMAGE_HEIGHT).rev() {
        write_progress_update(row, writer_err)?;
        for col in 0..IMAGE_WIDTH {
            let u = col as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = row as f64 / (IMAGE_HEIGHT - 1) as f64;
            let ray = Ray::new(ORIGIN, lower_left + u * HORIZONTAL + v * VERTICAL - ORIGIN);
            let pixel_color = get_ray_color(ray);
            write_pixel(writer, pixel_color)?;
        }
    }

    Ok(())
}

fn get_ray_color(ray: Ray) -> Color {
    let hit = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if let Hit::Some(t) = hit {
        let normal = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
        return 0.5 * Color::new(normal.x() + 1.0, normal.y() + 1.0, normal.z() + 1.0);
    }
    let direction = ray.direction().normalized();
    let t = 0.5 * (direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

enum Hit {
    Some(f64),
    None,
}

fn hit_sphere(center: Point3, radius: f64, ray: Ray) -> Hit {
    let co = ray.origin() - center;

    // Quadratic equation
    let a = ray.direction().length_squared();
    let half_b = Vec3::dot(co, ray.direction());
    let c = co.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return Hit::None;
    }

    Hit::Some((-half_b - discriminant.sqrt()) / a)
}
