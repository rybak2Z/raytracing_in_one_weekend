pub use std::io;
use std::io::{BufWriter, StderrLock, StdoutLock, Write};

use crate::config::{FILE_TYPE, IMAGE_HEIGHT, IMAGE_WIDTH, MAX_COLOR, PROGRESS_NUM_WIDTH};
use crate::vec3::Color;

pub type Writer<'a> = BufWriter<StdoutLock<'a>>;
pub type WriterErr<'a> = BufWriter<StderrLock<'a>>;

pub fn get_writers() -> (Writer<'static>, WriterErr<'static>) {
    let stdout = io::stdout().lock();
    let writer = BufWriter::new(stdout);

    let stderr = io::stderr().lock();
    let writer_err = BufWriter::new(stderr);

    (writer, writer_err)
}

pub fn write_meta_data(writer: &mut Writer) -> io::Result<()> {
    write!(
        writer,
        "{FILE_TYPE}\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n{MAX_COLOR}\n"
    )
}

pub fn finish_writers(writer: &mut Writer, writer_err: &mut WriterErr) -> io::Result<()> {
    writer.flush()?;
    writer_err.write_all(b"\nDone.\n")?;
    writer_err.flush()?;
    Ok(())
}

pub fn write_progress_update(row: u32, writer_err: &mut WriterErr) -> io::Result<()> {
    write!(
        writer_err,
        "\rScanlines remaining: {:0>width$}",
        row,
        width = PROGRESS_NUM_WIDTH as usize
    )?;
    writer_err.flush()?;
    Ok(())
}

pub fn write_pixel(writer: &mut Writer, pixel_color: Color) -> io::Result<()> {
    writeln!(
        writer,
        "{} {} {}",
        (255.0 * pixel_color.r()) as u32,
        (255.0 * pixel_color.g()) as u32,
        (255.0 * pixel_color.b()) as u32
    )
}
