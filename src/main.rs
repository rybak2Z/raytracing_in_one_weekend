use raytracing_in_one_weekend::config::*;
use raytracing_in_one_weekend::rendering::render;
use raytracing_in_one_weekend::read_scene::read_scene;
use raytracing_in_one_weekend::scene_building::build_scene;
use raytracing_in_one_weekend::writing::write_meta_data;

use std::io::{self, Error, ErrorKind};

fn main() -> io::Result<()> {
    let result = generate_config();
    if let Err(e) = result {
        print!("Error: {e}");
    }

    if *THREADS.get().unwrap() == 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "Invalid number of threads (needs to be greater or equal to 1).",
        ));
    }

    write_meta_data()?;

    let (world, camera) = match USE_BUILD_FUNCTION.get().unwrap() {
        true => build_scene(),
        false => read_scene()?
    };

    render(world, camera)?;
    eprintln!("\nDone.");

    Ok(())
}
