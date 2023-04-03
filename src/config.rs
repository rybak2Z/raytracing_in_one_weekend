pub mod set_up;

mod deserialization;

pub use set_up::set_up;

use once_cell::sync::OnceCell;

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
