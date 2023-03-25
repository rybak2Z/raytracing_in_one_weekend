use std::io::Write;
use std::io::{self, BufWriter};

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;
const FILE_TYPE: &str = "P3";
const MAX_COLOR: u32 = 255;

fn main() -> io::Result<()> {
    let stdout = io::stdout().lock();
    let mut writer = BufWriter::new(stdout);

    writer.write_all(format!("{FILE_TYPE}\n{HEIGHT} {WIDTH}\n{MAX_COLOR}\n").as_bytes())?;

    for row in (0..HEIGHT).rev() {
        for col in 0..WIDTH {
            let r = col as f32 / (WIDTH - 1) as f32;
            let g = row as f32 / (HEIGHT - 1) as f32;
            let b: f32 = 0.25;

            let r = (255.0 * r) as u32;
            let g = (255.0 * g) as u32;
            let b = (255.0 * b) as u32;
            
            let output = format!("{r} {g} {b}\n");
            writer.write_all(output.as_bytes())?;
        }
    }

    writer.flush()?;

    Ok(())
}
