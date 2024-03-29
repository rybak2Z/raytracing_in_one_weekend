use raytracing_in_one_weekend::color::Color;
use raytracing_in_one_weekend::config::{self, *};

use std::io::{self, BufWriter, Write};
use std::time::Instant;

fn main() -> anyhow::Result<()> {
    config::initialize()?;

    let mut stdout = BufWriter::new(io::stdout());
    write!(
        stdout,
        "{}\n{} {}\n{}\n",
        FILE_TYPE,
        image_width(),
        image_height(),
        MAX_VALUE,
    )?;

    let time_start = Instant::now();

    for row in 0..image_height() {
        for col in 0..image_width() {
            let color = Color::new(
                col as f32 / (image_width() - 1) as f32,
                row as f32 / (image_height() - 1) as f32,
                0.0,
            );

            write!(stdout, "{}", color.pixel_format())?;
        }

        writeln!(stdout)?;
        print_progress(row);
    }

    stdout.flush()?;
    print_finish(time_start);
    Ok(())
}

fn print_progress(row: u32) {
    let progress = row as f32 / (image_height() - 1) as f32;
    let lines_remaining = image_height() - (row + 1);
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
