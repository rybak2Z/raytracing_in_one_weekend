use crate::config::err_invalid_data;
use crate::rendering::{HittableList, camera::*, Vec3, Point3, Color};

use serde::Deserialize;

use std::io;

const SCENE_PATH: &str = "default_scene.json";

#[derive(Deserialize)]
struct Scene {
    camera: JsonCamera,
    materials: Vec<JsonMaterial>,
    objects: Vec<JsonSphere>,
}

impl Scene {
    fn validate(&self) -> io::Result<()> {
        self.camera.validate()?;
        for material in self.materials.iter() {
            material.validate()?;
        }
        for object in self.objects.iter() {
            object.validate()?;
        }
        Ok(())
    }
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

impl JsonCamera {
    fn validate(&self) -> io::Result<()> {
        todo!()
    }
}

#[derive(Deserialize)]
enum JsonMaterial {
    ReferenceToName(String),
    Literal(JsonMaterialLiteral),
}

impl JsonMaterial {
    fn validate(&self) -> io::Result<()> {
        todo!()
    }
}

#[derive(Deserialize)]
struct JsonMaterialLiteral {
    name: Option<String>,
    type_: String,
    color: JsonColor,
    refractive_index: Option<f64>,
    fuzziness: Option<f64>
}

impl JsonMaterialLiteral {
    fn validate(&self) -> io::Result<()> {
        todo!()
    }
}

#[derive(Deserialize)]
struct JsonColor {
    rgb: (f64, f64, f64),
    normalized: bool,
}

impl JsonColor {
    fn validate(&self) -> io::Result<()> {
        todo!()
    }
}

#[derive(Deserialize)]
struct JsonSphere {
    _name: Option<String>,
    coordinates: (f64, f64, f64),
    radius: f64,
    material: JsonMaterial,
}

impl JsonSphere {
    fn validate(&self) -> io::Result<()> {
        todo!()
    }
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
        return Err(err_invalid_data(&format!("Failed to deserialize {}", SCENE_PATH)));
    }

    Ok(scene.unwrap())
}

fn create_camera(json_camera: JsonCamera) -> io::Result<Camera> {
    todo!()
}

fn create_world(json_objects: Vec<JsonSphere>) -> io::Result<HittableList> {
    todo!()
}
