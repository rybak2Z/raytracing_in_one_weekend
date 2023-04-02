mod validation;

use crate::rendering::{camera::*, HittableList, Point3, Vec3};

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
#[serde(untagged)]
enum JsonMaterial {
    ReferenceToName(String),
    Literal(JsonMaterialLiteral),
}

#[derive(Deserialize)]
struct JsonMaterialLiteral {
    name: Option<String>,
    #[serde(rename = "type")] // "type" is a reserved keyword is rust
    type_: JsonMaterialOptions,
    color: Option<JsonColor>,
    refractive_index: Option<f64>,
    fuzziness: Option<f64>,
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
    let camera = create_camera(scene.camera);
    let world = create_world(scene.objects);

    Ok((world, camera))
}

fn read_scene_file() -> io::Result<Scene> {
    let file_contents = std::fs::read_to_string(SCENE_PATH)?;
    let scene = serde_json::from_str::<Scene>(&file_contents)?;

    Ok(scene)
}

fn create_camera(json_camera: JsonCamera) -> Camera {
    let coords = json_camera.look_from;
    let look_from = Point3::new(coords.x, coords.y, coords.z);
    let coords = json_camera.look_at;
    let look_at = Point3::new(coords.x, coords.y, coords.z);
    let vec = json_camera.view_up_direction;
    let view_up = Vec3::new(vec.x, vec.y, vec.z);

    let camera_config = CameraConfiguration {
        look_from,
        look_at,
        view_up,
        vertical_fov: json_camera.vertical_fov_degrees,
        aperture: json_camera.aperture,
        focus_distance: json_camera.focus_distance,
        focal_length: json_camera.focal_length,
    };

    Camera::new(camera_config)
}

fn create_world(json_objects: Vec<JsonSphere>) -> HittableList {
    let mut world = HittableList::default();
    for json_sphere in json_objects {
        world.add(Box::new(json_sphere.to_sphere()));
    }
    world
}
