use std::io::{Error, ErrorKind};

use raytracing_in_one_weekend::camera::Camera;
use raytracing_in_one_weekend::config;
use raytracing_in_one_weekend::rendering::*;
use raytracing_in_one_weekend::world_building::*;
use raytracing_in_one_weekend::writing::*;

fn main() -> io::Result<()> {
    if config::THREADS <= 0 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Invalid number of threads (needs to be greater or equal to 1)."),
        ));
    }

    write_meta_data()?;

    let (world, world_cam_config) = match config::WORLD_TYPE {
        WorldType::Custom1 => build_custom_1(),
        WorldType::Random1 => build_random_1(),
    };

    let camera = match config::USE_WORLD_SPECIFIC_CAM {
        true => Camera::new(world_cam_config),
        false => Camera::new(config::CAMERA_CONFIG),
    };

    render(world, camera)?;
    eprintln!("\nDone.");

    Ok(())
}
