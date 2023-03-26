pub use std::io::{self, BufWriter, StderrLock, StdoutLock, Write};

use crate::config::{FILE_TYPE, IMAGE_HEIGHT, IMAGE_WIDTH, MAX_COLOR};
use crate::vec3::Color;

pub fn get_writers() -> (
    BufWriter<StdoutLock<'static>>,
    BufWriter<StderrLock<'static>>,
) {
    let stdout = io::stdout().lock();
    let writer = BufWriter::new(stdout);

    let stderr = io::stderr().lock();
    let writer_err = BufWriter::new(stderr);

    (writer, writer_err)
}

pub fn write_meta_data(writer: &mut BufWriter<StdoutLock>) -> io::Result<()> {
    write!(
        writer,
        "{FILE_TYPE}\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n{MAX_COLOR}\n"
    )
}

pub fn finish_writers(
    writer: &mut BufWriter<StdoutLock>,
    writer_err: &mut BufWriter<StderrLock>,
) -> io::Result<()> {
    writer.flush()?;
    writer_err.write_all(b"\nDone.\n")?;
    writer_err.flush()?;
    Ok(())
}

pub fn write_progress_update(row: u32, writer_err: &mut BufWriter<StderrLock>) -> io::Result<()> {
    write!(writer_err, "\rScanlines remaining: {row}")?;
    writer_err.flush()?;
    Ok(())
}

pub fn write_pixel<W: Write>(writer: &mut W, pixel_color: Color) -> io::Result<()> {
    writeln!(
        writer,
        "{} {} {}",
        (255.0 * pixel_color.x()) as u32,
        (255.0 * pixel_color.y()) as u32,
        (255.0 * pixel_color.z()) as u32
    )
}
