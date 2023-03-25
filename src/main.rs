use std::io::Write;
use std::io::{self, BufWriter};

use raytracing_in_one_weekend::vec3::Color;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const FILE_TYPE: &str = "P3";
const MAX_COLOR: u32 = 255;

fn main() -> io::Result<()> {
    let stdout = io::stdout().lock();
    let mut writer = BufWriter::new(stdout);
    write!(writer, "{FILE_TYPE}\n{HEIGHT} {WIDTH}\n{MAX_COLOR}\n")?;

    let stderr = io::stderr().lock();
    let mut writer_err = BufWriter::new(stderr);

    for row in (0..HEIGHT).rev() {
        write!(writer_err, "\rScanlines remaining: {row}")?;
        writer_err.flush()?;

        for col in 0..WIDTH {
            let pixel_color = Color::new(
                col as f64 / (WIDTH - 1) as f64,
                row as f64 / (HEIGHT - 1) as f64,
                0.25,
            );

            write_pixel(&mut writer, pixel_color)?;
        }
    }

    writer.flush()?;

    writer_err.write_all(b"\nDone.\n")?;
    writer_err.flush()?;

    Ok(())
}

fn write_pixel<W: Write>(writer: &mut W, pixel_color: Color) -> io::Result<()> {
    write!(
        writer,
        "{} {} {}\n",
        (255.0 * pixel_color.x()) as u32,
        (255.0 * pixel_color.y()) as u32,
        (255.0 * pixel_color.z()) as u32
    )
}
