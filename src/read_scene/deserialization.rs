mod to_internal_types;
mod validation;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Scene {
    pub camera: JsonCamera,
    pub materials: Vec<JsonMaterial>,
    pub objects: Vec<JsonSphere>,
}

#[derive(Deserialize)]
pub struct JsonVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Deserialize)]
pub struct JsonCamera {
    pub look_from: JsonVec3,
    pub look_at: JsonVec3,
    pub view_up_direction: JsonVec3,
    pub vertical_fov_degrees: f64,
    pub aperture: f64,
    pub focus_distance: Option<f64>,
    pub focal_length: f64,
    #[serde(default)]
    pub start_time: f64,
    #[serde(default = "end_time_default")]
    pub end_time: f64,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum JsonMaterial {
    ReferenceToName(String),
    Literal(JsonMaterialLiteral),
}

#[derive(Deserialize)]
pub struct JsonMaterialLiteral {
    pub name: Option<String>,
    #[serde(rename = "type")] // "type" is a reserved keyword is rust
    pub type_: JsonMaterialOptions,
    pub color: Option<JsonColor>,
    pub refractive_index: Option<f64>,
    pub fuzziness: Option<f64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum JsonMaterialOptions {
    Diffuse,
    Metal,
    Dialectric,
}

impl std::fmt::Display for JsonMaterialOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize)]
pub struct JsonColor {
    pub rgb: (f64, f64, f64),
    pub normalized: bool,
}

#[derive(Deserialize)]
pub struct JsonSphere {
    pub _name: Option<String>,
    pub coordinates: (f64, f64, f64),
    pub movement: Option<JsonMovement>,
    pub radius: f64,
    pub material: JsonMaterial,
}

#[derive(Deserialize)]
pub struct JsonMovement {
    pub target: (f64, f64, f64),
    #[serde(default)]
    pub start_time: f64,
    #[serde(default = "end_time_default")]
    pub end_time: f64,
}

fn end_time_default() -> f64 {
    1.0
}
