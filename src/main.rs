use raytracing_in_one_weekend::config::{self, THREADS, USE_BUILD_FUNCTION};
use raytracing_in_one_weekend::read_scene::read_scene;
use raytracing_in_one_weekend::rendering::{render, BvhNode};
use raytracing_in_one_weekend::scene_building::build_scene;
use raytracing_in_one_weekend::writing::write_meta_data;

use std::io::{self, Error, ErrorKind};

fn main() -> io::Result<()> {
    config::set_up()?;

    if *THREADS.get().unwrap() == 0 {
        return Err(Error::new(
            ErrorKind::Other,
            "Invalid number of threads (needs to be greater or equal to 1).",
        ));
    }

    write_meta_data()?;

    let (world, camera) = match USE_BUILD_FUNCTION.get().unwrap() {
        true => build_scene(),
        false => read_scene()?,
    };
    let bvh = BvhNode::new(&world, camera.get_start_time(), camera.get_end_time());

    let time_taken = render(bvh, camera)?;
    eprintln!("\nDone. (time taken: {time_taken})");

    Ok(())
}
