use once_cell::sync::OnceCell;
use serde::Deserialize;

use std::io::{self, ErrorKind::InvalidData};

const CONFIG_PATH: &str = "config.toml";

pub static ASPECT_RATIO: OnceCell<f64> = OnceCell::new();
pub static IMAGE_WIDTH: OnceCell<u32> = OnceCell::new();
pub static IMAGE_HEIGHT: OnceCell<u32> = OnceCell::new();
pub static PIXELS_TOTAL: OnceCell<u32> = OnceCell::new();
pub static SAMPLES_PER_PIXEL: OnceCell<u32> = OnceCell::new();
pub static MAX_CHILD_RAYS: OnceCell<u32> = OnceCell::new();
pub static THREADS: OnceCell<u32> = OnceCell::new();
pub static USE_MAIN_THREAD_FOR_RENDERING: OnceCell<bool> = OnceCell::new();
pub static UPDATE_EVERY_N_PIXELS: OnceCell<u32> = OnceCell::new();
pub static WRITING_BUFFER_START_CAPACITY: OnceCell<usize> = OnceCell::new();

#[derive(Deserialize, Debug)]
struct TomlConfiguration {
    image: TomlImageConfiguration,
    rendering: TomlRenderingConfiguration,
}

#[derive(Deserialize, Debug)]
struct TomlImageConfiguration {
    aspect_ratio: Option<(u32, u32)>,
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Deserialize, Debug)]
struct TomlRenderingConfiguration {
    samples_per_pixel: u32,
    max_child_ray_depth: u32,
    threads: u32,
    main_thread_for_render: bool,
    update_frequency: u32,
    writing_buffer_capacity: usize,
}

fn err_invalid_data(message: &str) -> io::Error {
    io::Error::new(InvalidData, message)
}

fn read_config_file() -> io::Result<TomlConfiguration> {
    let file_contents = std::fs::read_to_string(CONFIG_PATH)?;
    let config = toml::from_str::<TomlConfiguration>(&file_contents);
    if config.is_err() {
        return Err(err_invalid_data(&format!(
            "Failed to deserialize {CONFIG_PATH}"
        )));
    }

    Ok(config.unwrap())
}

fn determine_image_settings(image_config: TomlImageConfiguration) -> io::Result<(f64, u32, u32)> {
    let (aspect_ratio, width, height) = (
        image_config.aspect_ratio,
        image_config.width,
        image_config.height,
    );
    match (aspect_ratio, width, height) {
        (Some((x, y)), Some(w), None) if x > 0 && y > 0 => {
            let ratio = x as f64 / y as f64;
            let h = (w as f64 / ratio).round() as u32;
            Ok((ratio, w, h))
        }
        (Some((x, y)), None, Some(h)) if x > 0 && y > 0 => {
            let ratio = x as f64 / y as f64;
            let w = (h as f64 * ratio).round() as u32;
            Ok((ratio, w, h))
        }
        (None, Some(w), Some(h)) => Ok((w as f64 / h as f64, w, h)),
        (_, _, _) => return Err(err_invalid_data("Invalid image settings")),
    }
}

pub fn generate_config() -> io::Result<()> {
    let TomlConfiguration { image, rendering } = read_config_file()?;
    let (aspect_ratio, width, height) = determine_image_settings(image)?;
    let pixels_total = width * height;

    let samples_per_pixel = rendering.samples_per_pixel;
    if samples_per_pixel == 0 {
        eprintln!("Warning: The number of samples per pixel is set to 0. The result will probably not look too interesting...");
    }

    let threads = rendering.threads;
    if threads == 0 {
        return Err(err_invalid_data(
            "Number of threads must be greater or equal to 1.",
        ));
    }

    let max_child_ray_depth = rendering.max_child_ray_depth;
    let main_thread_for_render = rendering.main_thread_for_render;
    let update_frequency = rendering.update_frequency;
    let writing_buffer_capacity = rendering.writing_buffer_capacity;

    let successes = [
        ASPECT_RATIO.set(aspect_ratio).is_ok(),
        IMAGE_WIDTH.set(width).is_ok(),
        IMAGE_HEIGHT.set(height).is_ok(),
        PIXELS_TOTAL.set(pixels_total).is_ok(),
        SAMPLES_PER_PIXEL.set(samples_per_pixel).is_ok(),
        MAX_CHILD_RAYS.set(max_child_ray_depth).is_ok(),
        THREADS.set(threads).is_ok(),
        USE_MAIN_THREAD_FOR_RENDERING.set(main_thread_for_render).is_ok(),
        UPDATE_EVERY_N_PIXELS.set(update_frequency).is_ok(),
        WRITING_BUFFER_START_CAPACITY.set(writing_buffer_capacity).is_ok(),
    ];

    if successes.iter().any(|success| !success) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "An unexpected error occured.",
        ));
    }

    Ok(())
}

// File format
pub const FILE_TYPE: &str = "P3";
pub const MAX_COLOR: u32 = 255;
