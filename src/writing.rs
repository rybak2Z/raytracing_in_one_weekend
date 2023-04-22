use crate::config::*;
use crate::rendering::Color;

use std::io::{self, BufWriter, Error, ErrorKind, StderrLock, StdoutLock, Write};
use std::time::{Duration, Instant};

pub type Writer<'a> = BufWriter<StdoutLock<'a>>;
pub type WriterErr<'a> = BufWriter<StderrLock<'a>>;

// File format
pub const FILE_TYPE: &str = "P3";
pub const MAX_COLOR: u32 = 255;

const TIME_LEFT_UPDATE_FREQUENCY: f64 = 1.0;  // in seconds

pub struct WritingSynchronizer<'a> {
    buffer: Vec<(Color, u32)>,
    next_to_write: i32,
    writer: Writer<'a>,
    writer_err: WriterErr<'a>,
    pixels_written: u32,
    update_counter: u32,
    update_every: u32,
    last_write: Instant,
    time_passed: Duration,
    time_remaining_formatted: String,
}

impl WritingSynchronizer<'_> {
    pub fn new() -> WritingSynchronizer<'static> {
        WritingSynchronizer {
            buffer: Vec::with_capacity(*WRITING_BUFFER_START_CAPACITY.get().unwrap()),
            next_to_write: (PIXELS_TOTAL.get().unwrap() - 1) as i32,
            writer: BufWriter::new(io::stdout().lock()),
            writer_err: BufWriter::new(io::stderr().lock()),
            pixels_written: 0,
            update_counter: 0,
            update_every: *UPDATE_EVERY_N_PIXELS.get().unwrap(),
            last_write: Instant::now(),
            time_passed: Duration::default(),
            time_remaining_formatted: String::new(),
        }
    }

    pub fn write(&mut self, pixel_color: Color, row_from_bottom: u32, col: u32) -> io::Result<()> {
        self.print_progress()?;

        self.add_to_buffer(pixel_color, row_from_bottom, col);
        self.buffer.sort_by_key(|entry| entry.1);

        while !self.buffer.is_empty() {
            let success = self.try_to_write()?;
            if !success {
                break;
            }
        }

        Ok(())
    }

    pub fn total_time_taken(&self) -> String {
        format_duration(self.time_passed)
    }

    fn add_to_buffer(&mut self, pixel_color: Color, row_from_bottom: u32, col: u32) {
        let row = (IMAGE_HEIGHT.get().unwrap() - 1) - row_from_bottom;
        let pixel_index = row * IMAGE_WIDTH.get().unwrap() + col;

        // So that the first pixels that should be written are at the end of the vector when sorted (for pop())
        let reversed_index = PIXELS_TOTAL.get().unwrap() - 1 - pixel_index;

        self.buffer.push((pixel_color, reversed_index));
    }

    fn print_progress(&mut self) -> io::Result<()> {
        self.update_counter += 1;
        self.pixels_written += 1;
        if self.update_counter % self.update_every == 0 {
            let pixels_total = *PIXELS_TOTAL.get().unwrap();
            let relative_progress =
                self.pixels_written as f64 / pixels_total as f64;
            self.update_remaining_time_estimation(pixels_total);

            write_progress_update(relative_progress, &self.time_remaining_formatted, &mut self.writer_err)?;
            self.update_counter = 0;
        }

        Ok(())
    }

    fn update_remaining_time_estimation(&mut self, pixels_total: u32) {
        let delta_t = Instant::now() - self.last_write;
        if delta_t.as_secs_f64() > TIME_LEFT_UPDATE_FREQUENCY {
            self.time_passed += delta_t;
            
            let seconds_per_pixel = self.time_passed / self.pixels_written;
            let pixels_left = pixels_total - self.pixels_written;
            let time_left = seconds_per_pixel * pixels_left;
            self.time_remaining_formatted = format_duration(time_left);
            self.last_write = Instant::now();
        }
    }

    fn try_to_write(&mut self) -> io::Result<bool> {
        let next_pixel_index = self.buffer[self.buffer.len() - 1].1;
        if next_pixel_index != self.next_to_write as u32 {
            return Ok(false);
        }

        let color = self.buffer.pop().unwrap().0;
        write_pixel(&mut self.writer, color)?;
        self.next_to_write -= 1;

        Ok(true)
    }

    pub fn all_data_written(&self) -> bool {
        self.buffer.is_empty() && self.next_to_write == -1
    }

    pub fn finish_writing(&mut self) -> io::Result<()> {
        self.writer.flush()?;

        if !self.all_data_written() {
            return Err(Error::new(
                ErrorKind::Other,
                "Error: Failed to write all data to output.",
            ));
        }

        Ok(())
    }
}

pub fn write_meta_data() -> io::Result<()> {
    write!(
        std::io::stdout(),
        "{}\n{} {}\n{}\n",
        FILE_TYPE,
        IMAGE_WIDTH.get().unwrap(),
        IMAGE_HEIGHT.get().unwrap(),
        MAX_COLOR,
    )
}

pub fn write_progress_update(relative_progress: f64, time_left: &str, writer_err: &mut WriterErr) -> io::Result<()> {
    write!(
        writer_err,
        "\rRendering... {:.2} % (time remaining: {})",
        relative_progress * 100.0,
        time_left,
    )?;
    writer_err.flush()?;
    Ok(())
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs() % 60;
    let minutes = (duration.as_secs() / 60) % 60;
    let mut hours = (duration.as_secs() / 60 / 60) % 60;
    if hours > 99 {
        hours = 99;
    }

    format!("{:0>2}h{:0>2}m{:0>2}s", hours, minutes, seconds)
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
