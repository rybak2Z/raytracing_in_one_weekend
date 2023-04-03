mod deserialization;

use deserialization::*;

use once_cell::sync::OnceCell;

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
pub static USE_BUILD_FUNCTION: OnceCell<bool> = OnceCell::new();

pub fn err_invalid_data(message: &str) -> io::Error {
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
        (_, _, _) => Err(err_invalid_data("Invalid image settings")),
    }
}

pub fn generate_config() -> io::Result<()> {
    let TomlConfiguration {
        image,
        rendering,
        other,
    } = read_config_file()?;
    let (aspect_ratio, width, height) = determine_image_settings(image)?;
    let pixels_total = width * height;

    validate_data(rendering.samples_per_pixel, rendering.threads)?;

    let successes = [
        ASPECT_RATIO.set(aspect_ratio).is_ok(),
        IMAGE_WIDTH.set(width).is_ok(),
        IMAGE_HEIGHT.set(height).is_ok(),
        PIXELS_TOTAL.set(pixels_total).is_ok(),
        SAMPLES_PER_PIXEL.set(rendering.samples_per_pixel).is_ok(),
        MAX_CHILD_RAYS.set(rendering.max_child_ray_depth).is_ok(),
        THREADS.set(rendering.threads).is_ok(),
        USE_MAIN_THREAD_FOR_RENDERING
            .set(rendering.main_thread_for_render)
            .is_ok(),
        UPDATE_EVERY_N_PIXELS
            .set(rendering.update_frequency)
            .is_ok(),
        WRITING_BUFFER_START_CAPACITY
            .set(rendering.writing_buffer_capacity)
            .is_ok(),
        USE_BUILD_FUNCTION.set(other.use_build_function).is_ok(),
    ];

    if successes.iter().any(|success| !success) {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "An unexpected error occured.",
        ));
    }

    Ok(())
}

fn validate_data(samples_per_pixel: u32, threads: u32) -> io::Result<()> {
    if samples_per_pixel == 0 {
        eprintln!("Warning: The number of samples per pixel is set to 0. The result will probably not look too interesting...");
    }

    if threads == 0 {
        return Err(err_invalid_data(
            "Number of threads must be greater or equal to 1.",
        ));
    }

    Ok(())
}
