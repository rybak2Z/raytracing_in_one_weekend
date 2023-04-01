mod validation;

use crate::config::err_invalid_data;
use crate::rendering::{camera::*, Color, HittableList, Point3, Vec3};

use serde::Deserialize;

use std::io;

const SCENE_PATH: &str = "default_scene.json";

#[derive(Deserialize)]
struct Scene {
    camera: JsonCamera,
    materials: Vec<JsonMaterial>,
    objects: Vec<JsonSphere>,
}

#[derive(Deserialize)]
struct JsonVec3 {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Deserialize)]
struct JsonCamera {
    look_from: JsonVec3,
    look_at: JsonVec3,
    view_up_direction: JsonVec3,
    vertical_fov_degrees: f64,
    aperture: f64,
    focus_distance: Option<f64>,
    focal_length: f64,
}

#[derive(Deserialize)]
enum JsonMaterial {
    ReferenceToName(String),
    Literal(JsonMaterialLiteral),
}

#[derive(Deserialize)]
struct JsonMaterialLiteral {
    name: Option<String>,
    type_: String,
    color: JsonColor,
    refractive_index: Option<f64>,
    fuzziness: Option<f64>,
}

#[derive(Deserialize)]
struct JsonColor {
    rgb: (f64, f64, f64),
    normalized: bool,
}

#[derive(Deserialize)]
struct JsonSphere {
    _name: Option<String>,
    coordinates: (f64, f64, f64),
    radius: f64,
    material: JsonMaterial,
}

pub fn generate_scene() -> io::Result<(HittableList, Camera)> {
    let scene = read_scene_file()?;
    scene.validate()?;
    let camera = create_camera(scene.camera)?;
    let world = create_world(scene.objects)?;

    Ok((world, camera))
}

fn read_scene_file() -> io::Result<Scene> {
    let file_contents = std::fs::read_to_string(SCENE_PATH)?;
    let scene = serde_json::from_str::<Scene>(&file_contents);
    if scene.is_err() {
        return Err(err_invalid_data(&format!(
            "Failed to deserialize {}",
            SCENE_PATH
        )));
    }

    Ok(scene.unwrap())
}

fn create_camera(json_camera: JsonCamera) -> io::Result<Camera> {
    todo!()
}

fn create_world(json_objects: Vec<JsonSphere>) -> io::Result<HittableList> {
    todo!()
}
