//! This module contains globally used configuration data. Due to the
//! limitations of Rust's const system, some the relevant data cannot be
//! stored in const variables. So instead, they are mutable static variables.
//!
//! Since mutable static variables require unsafe blocks for reading and
//! writing, this module's API only offers safe wrapper methods for the sake of
//! brevity. The unsafe operations should always be safe however, because this
//! program is not (yet) multi-threaded.

use anyhow::{self, bail};
use serde::Deserialize;
use toml;

use std::io::Read;

pub const FILE_TYPE: &str = "P3";
pub const MAX_VALUE: u32 = 255;

const CONFIGURATION_FILE_NAME: &str = "config.toml";

static mut IMAGE_WIDTH: u32 = 0;
static mut IMAGE_HEIGHT: u32 = 0;

pub fn image_width() -> u32 {
    unsafe { IMAGE_WIDTH }
}

pub fn image_height() -> u32 {
    unsafe { IMAGE_HEIGHT }
}

pub fn aspect_ratio() -> f32 {
    unsafe { ASPECT_RATIO }
}

static mut ASPECT_RATIO: f32 = 0.0;

pub fn initialize() -> anyhow::Result<()> {
    let toml_config = read_toml_config_file()?;

    unsafe {
        IMAGE_WIDTH = toml_config.image_width();
        IMAGE_HEIGHT = toml_config.image_height();
        ASPECT_RATIO = toml_config.aspect_ratio();
    }

    Ok(())
}

fn read_toml_config_file() -> anyhow::Result<TomlConfig> {
    let mut config_file = std::fs::File::open(CONFIGURATION_FILE_NAME)?;
    let mut contents = String::new();
    config_file.read_to_string(&mut contents)?;
    let unvalidated_config: UnvalidatedTomlConfig = toml::from_str(&contents)?;
    unvalidated_config.validate()
}

#[derive(Deserialize)]
struct UnvalidatedTomlConfig {
    // The types are wrapped in options because if two values are given, the
    // third value can be calculated. So, the user is expected to specify
    // exactly two values and leave/comment the third one out. Any other
    // configuration results in an error.
    image_width: Option<u32>,
    image_height: Option<u32>,
    aspect_ratio: Option<(u32, u32)>, // (width, height)
}

impl UnvalidatedTomlConfig {
    pub fn validate(self) -> anyhow::Result<TomlConfig> {
        let (image_width, image_height, aspect_ratio) = match (self.image_width, self.image_height, self.aspect_ratio) {
            (Some(width), Some(height), None) => (
                width,
                height,
                (width, height),
            ),
            (Some(width), None, Some(aspect_ratio)) => (
                width,
                UnvalidatedTomlConfig::image_height(width, aspect_ratio),
                aspect_ratio,
            ),
            (None, Some(height), Some(aspect_ratio)) => (
                UnvalidatedTomlConfig::image_width(height, aspect_ratio),
                height,
                aspect_ratio,
            ),
            _ => bail!("Invalid configuration: Exactly (!) two fields out of 'image_width', 'image_height', and 'aspect_ratio' must be defined."),
        };

        let aspect_ratio = aspect_ratio.0 as f32 / aspect_ratio.1 as f32;

        Ok(TomlConfig {
            image_width,
            image_height,
            aspect_ratio,
        })
    }

    fn image_width(image_height: u32, aspect_ratio: (u32, u32)) -> u32 {
        let aspect_ratio = aspect_ratio.0 as f32 / aspect_ratio.1 as f32;
        let width = image_height as f32 * aspect_ratio;
        width.round() as u32
    }

    fn image_height(image_width: u32, aspect_ratio: (u32, u32)) -> u32 {
        let aspect_ratio = aspect_ratio.0 as f32 / aspect_ratio.1 as f32;
        let height = image_width as f32 / aspect_ratio;
        height.round() as u32
    }
}

struct TomlConfig {
    image_width: u32,
    image_height: u32,
    aspect_ratio: f32,
}

impl TomlConfig {
    fn image_width(&self) -> u32 {
        self.image_width
    }

    fn image_height(&self) -> u32 {
        self.image_height
    }

    fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }
}
