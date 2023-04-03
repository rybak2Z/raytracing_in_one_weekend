use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct TomlConfiguration {
    pub image: TomlImageConfiguration,
    pub rendering: TomlRenderingConfiguration,
    pub other: TomlOtherConfiguration,
}

#[derive(Deserialize, Debug)]
pub struct TomlImageConfiguration {
    pub aspect_ratio: Option<(u32, u32)>,
    pub width: Option<u32>,
    pub height: Option<u32>,
}

#[derive(Deserialize, Debug)]
pub struct TomlRenderingConfiguration {
    pub samples_per_pixel: u32,
    pub max_child_ray_depth: u32,
    pub threads: u32,
    pub main_thread_for_render: bool,
    pub update_frequency: u32,
    pub writing_buffer_capacity: usize,
}

#[derive(Deserialize, Debug)]
pub struct TomlOtherConfiguration {
    pub use_build_function: bool,
}
