use raytracing_in_one_weekend::config::*;
use raytracing_in_one_weekend::rendering::{camera::Camera, render};
use raytracing_in_one_weekend::world_building::*;
use raytracing_in_one_weekend::writing::write_meta_data;

use std::io::{self, Error, ErrorKind};

fn main() -> io::Result<()> {
    let result = generate_config();
    if let Err(e) = result {
        print!("Error: {e}");
    }

    if *THREADS.get().unwrap() <= 0 {
        return Err(Error::new(
            ErrorKind::Other,
            format!("Invalid number of threads (needs to be greater or equal to 1)."),
        ));
    }

    write_meta_data()?;

    let (world, world_cam_config) = build_random_1();
    let camera = Camera::new(world_cam_config);

    render(world, camera)?;
    eprintln!("\nDone.");

    Ok(())
}
