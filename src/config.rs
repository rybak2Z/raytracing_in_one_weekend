use crate::camera::CameraConfiguration;
use crate::vec3::{Point3, Vec3};
use crate::world_building::WorldType;

// Image
pub const ASCPECT_RATIO: f64 = 3.0 / 2.0;
pub const IMAGE_WIDTH: u32 = 100;
pub const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASCPECT_RATIO) as u32;
pub const PROGRESS_NUM_WIDTH: u32 = IMAGE_HEIGHT.ilog10() + 1;
pub const TOTAL_NUM_PIXELS: u32 = IMAGE_HEIGHT * IMAGE_WIDTH;

// World
pub const WORLD_TYPE: WorldType = WorldType::Custom1;

// Rendering
pub const SAMPLES_PER_PIXEL: u32 = 500;
pub const MAX_DEPTH: i32 = 50;
pub const THREADS: u32 = 6; // total number (0 won't work)
pub const USE_MAIN_THREAD_FOR_RENDERING: bool = false;
pub const UPDATE_PROGRESS_EVERY_N_PIXELS: u32 = 10;
pub const WRITING_BUFFER_START_CAPACITY: usize = 32;

// Camera
pub const USE_WORLD_SPECIFIC_CAM: bool = true; // if false, the settings in here are used
const LOOK_FROM: Point3 = Point3::new(13.0, 2.0, 3.0);
const LOOK_AT: Point3 = Point3::new(0.0, 0.0, 0.0);
const VIEW_UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
const VERTICAL_FOV: f64 = 20.0; // in degrees
const APERTURE: f64 = 0.1;
const FOCUS_DISTANCE: Option<f64> = Some(10.0);
const FOCAL_LENGTH: f64 = 1.0;
pub const CAMERA_CONFIG: CameraConfiguration = CameraConfiguration {
    look_from: LOOK_FROM,
    look_at: LOOK_AT,
    view_up: VIEW_UP,
    vertical_fov: VERTICAL_FOV,
    aperture: APERTURE,
    focus_distance: FOCUS_DISTANCE,
    focal_length: FOCAL_LENGTH,
};

// File format
pub const FILE_TYPE: &str = "P3";
pub const MAX_COLOR: u32 = 255;
