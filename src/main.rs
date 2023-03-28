use raytracing_in_one_weekend::camera::Camera;
use raytracing_in_one_weekend::config::{CAMERA_CONFIG, USE_WORLD_SPECIFIC_CAM, WORLD_TYPE};
use raytracing_in_one_weekend::rendering::*;
use raytracing_in_one_weekend::world_building::*;
use raytracing_in_one_weekend::writing::*;

fn main() -> io::Result<()> {
    let (mut writer, mut writer_err) = get_writers();
    write_meta_data(&mut writer)?;

    let (world, world_cam_config) = match WORLD_TYPE {
        WorldType::Custom1 => build_custom_1(),
        WorldType::Random1 => build_random_1(),
    };
    let camera = if USE_WORLD_SPECIFIC_CAM {
        Camera::new(world_cam_config)
    } else {
        Camera::new(CAMERA_CONFIG)
    };

    render(&world, camera, &mut writer, &mut writer_err)?;

    finish_writers(&mut writer, &mut writer_err)?;

    Ok(())
}
