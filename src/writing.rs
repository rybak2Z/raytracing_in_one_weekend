pub use std::io;
use std::io::{BufWriter, StderrLock, StdoutLock, Write};

use crate::config::{FILE_TYPE, IMAGE_HEIGHT, IMAGE_WIDTH, MAX_COLOR, PROGRESS_NUM_WIDTH};
use crate::vec3::Color;

pub type Writer<'a> = BufWriter<StdoutLock<'a>>;
pub type WriterErr<'a> = BufWriter<StderrLock<'a>>;

pub struct WritingSynchronizer<'a> {
    buffer: Vec<(Color, u32)>,
    next_to_write: u32,
    writer: BufWriter<StdoutLock<'a>>,
}

impl WritingSynchronizer<'_> {
    pub fn new() -> WritingSynchronizer<'static> {
        let buffer = Vec::with_capacity(16);
        let next_to_write: u32 = IMAGE_WIDTH * IMAGE_HEIGHT - 1;
        let stdout = io::stdout().lock();
        let writer = BufWriter::new(stdout);
        WritingSynchronizer { buffer, next_to_write, writer }
    }

    pub fn write(&mut self, pixel_color: Color, row_from_bottom: u32, col: u32) -> io::Result<()> {
        self.add_to_buffer(pixel_color, row_from_bottom, col);

        self.buffer.sort_by_key(|entry| entry.1);
        while !self.buffer.is_empty() {
            let last_index = self.buffer.len() - 1;
            if self.buffer[last_index].1 != self.next_to_write {
                break;
            }
            let color = self.buffer.pop().unwrap().0;
            write_pixel(&mut self.writer, color)?;
            self.next_to_write -= 1;
        }

        Ok(())
    }

    fn add_to_buffer(&mut self, pixel_color: Color, row_from_bottom: u32, col: u32) {
        let row = (IMAGE_HEIGHT - 1) - row_from_bottom;
        let pixel_index = row * IMAGE_WIDTH + col;
        // So that the first pixels to be popped are at the end of the vector when sorted
        let reversed_index = IMAGE_HEIGHT * IMAGE_WIDTH - 1 - pixel_index;
        self.buffer.push((pixel_color, reversed_index));
    }
}

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
    let relative_progress = (IMAGE_HEIGHT - row) as f64 / IMAGE_HEIGHT as f64;
    write!(
        writer_err,
        "\rRendering... {0:.2} % (scanlines remaining: {1:0>width$})",
        relative_progress * 100.0,
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
