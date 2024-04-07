const FILE_TYPE: &str = "P3";

use crate::color::{Color, MAX_VALUE};

use std::io::{self, BufWriter, Stdout, Write};
use std::time::Instant;

pub struct FileWriter {
    stdout: BufWriter<Stdout>,
}

impl FileWriter {
    pub fn new(image_width: u32, image_height: u32) -> io::Result<Self> {
        let mut stdout = BufWriter::new(io::stdout());

        // Write metadata
        write!(
            stdout,
            "{}\n{} {}\n{}\n",
            FILE_TYPE, image_width, image_height, MAX_VALUE
        )?;

        Ok(Self { stdout })
    }

    pub fn write_pixel(&mut self, pixel_color: Color, samples_per_pixel: u32) -> io::Result<()> {
        write!(
            self.stdout,
            "{}",
            pixel_color.pixel_format(samples_per_pixel)
        )
    }
}

impl Drop for FileWriter {
    fn drop(&mut self) {
        let result = self.stdout.flush();
        if let Err(error) = result {
            eprintln!("Warning: an error occured while flushing FileWriter: {error}");
        }
    }
}

pub struct ProgressWriter {
    start: Instant,
}

impl ProgressWriter {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn print_progress(&self, row: u32, image_height: u32) {
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
}

impl Drop for ProgressWriter {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        let seconds = duration.as_secs();
        let minutes = seconds / 60;
        let rest_seconds = seconds % 60;
        eprintln!("\nFinished after {:02}m{:02}s", minutes, rest_seconds);
    }
}
