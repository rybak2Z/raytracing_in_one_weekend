use raytracing_in_one_weekend::rendering::render;
use raytracing_in_one_weekend::writing::*;

fn main() -> io::Result<()> {
    let (mut writer, mut writer_err) = get_writers();
    write_meta_data(&mut writer)?;
    render(&mut writer, &mut writer_err)?;
    finish_writers(&mut writer, &mut writer_err)?;
    Ok(())
}
