use raytracing_in_one_weekend::vec3::Color;
use raytracing_in_one_weekend::writing::*;

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

fn get_pixel_color(row: u32, col: u32) -> Color {
    Color::new(
        col as f64 / (WIDTH - 1) as f64,
        row as f64 / (HEIGHT - 1) as f64,
        0.25,
    )
}
