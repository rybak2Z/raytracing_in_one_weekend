use std::io::{self, BufWriter, StderrLock, StdoutLock, Write};

use raytracing_in_one_weekend::vec3::Color;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const FILE_TYPE: &str = "P3";
const MAX_COLOR: u32 = 255;

fn main() -> io::Result<()> {
    let (mut writer, mut writer_err) = get_writers();
    write!(writer, "{FILE_TYPE}\n{HEIGHT} {WIDTH}\n{MAX_COLOR}\n")?;
    render(&mut writer, &mut writer_err)?;
    finish_writers(&mut writer, &mut writer_err)?;

    Ok(())
}

fn get_writers() -> (
    BufWriter<StdoutLock<'static>>,
    BufWriter<StderrLock<'static>>,
) {
    let stdout = io::stdout().lock();
    let writer = BufWriter::new(stdout);

    let stderr = io::stderr().lock();
    let writer_err = BufWriter::new(stderr);

    (writer, writer_err)
}

fn finish_writers(
    writer: &mut BufWriter<StdoutLock>,
    writer_err: &mut BufWriter<StderrLock>,
) -> io::Result<()> {
    writer.flush()?;
    writer_err.write_all(b"\nDone.\n")?;
    writer_err.flush()?;
    Ok(())
}

fn render(
    writer: &mut BufWriter<StdoutLock>,
    writer_err: &mut BufWriter<StderrLock>,
) -> io::Result<()> {
    for row in (0..HEIGHT).rev() {
        write_progress_update(row, writer_err)?;
        for col in 0..WIDTH {
            let pixel_color = get_pixel_color(row, col);
            write_pixel(writer, pixel_color)?;
        }
    }

    Ok(())
}

fn write_progress_update(row: u32, writer_err: &mut BufWriter<StderrLock>) -> io::Result<()> {
    write!(writer_err, "\rScanlines remaining: {row}")?;
    writer_err.flush()?;
    Ok(())
}

fn get_pixel_color(row: u32, col: u32) -> Color {
    Color::new(
        col as f64 / (WIDTH - 1) as f64,
        row as f64 / (HEIGHT - 1) as f64,
        0.25,
    )
}

fn write_pixel<W: Write>(writer: &mut W, pixel_color: Color) -> io::Result<()> {
    writeln!(
        writer,
        "{} {} {}",
        (255.0 * pixel_color.x()) as u32,
        (255.0 * pixel_color.y()) as u32,
        (255.0 * pixel_color.z()) as u32
    )
}
