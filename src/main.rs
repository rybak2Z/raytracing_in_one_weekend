use std::io::{self, BufWriter, Write};
use std::time::Instant;

const FILE_TYPE: &str = "P3";
const MAX_VALUE: u32 = 255;
const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() -> io::Result<()> {
    let mut stdout = BufWriter::new(io::stdout());
    write!(
        stdout,
        "{FILE_TYPE}\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n{MAX_VALUE}\n"
    )?;

    let time_start = Instant::now();

    for row in 0..IMAGE_HEIGHT {
        for col in 0..IMAGE_WIDTH {
            let red = col as f32 / (IMAGE_WIDTH - 1) as f32;
            let green = row as f32 / (IMAGE_HEIGHT - 1) as f32;
            let blue = 0;

            let red = (red * MAX_VALUE as f32) as u32;
            let green = (green * MAX_VALUE as f32) as u32;

            write!(stdout, "{red} {green} {blue} ")?;
        }

        writeln!(stdout)?;
        print_progress(row);
    }

    stdout.flush()?;
    print_finish(time_start);
    Ok(())
}

fn print_progress(row: u32) {
    let progress = row as f32 / (IMAGE_HEIGHT - 1) as f32;
    let lines_remaining = IMAGE_HEIGHT - (row + 1);
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
