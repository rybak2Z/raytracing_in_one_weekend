use crate::config::err_invalid_data;
use crate::rendering::{camera::*, Color, HittableList, Point3, Vec3};

use serde::Deserialize;

use std::io;

const SCENE_PATH: &str = "default_scene.json";

static mut DEFINED_MATERIALS: Vec<String> = Vec::new();

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
        if self.vertical_fov_degrees <= 0.0 || self.vertical_fov_degrees >= 180.0 {
            return Err(err_invalid_data("FOV must be between 0 and 180 degrees"));
        }
        if self.aperture <= 0.0 {
            return Err(err_invalid_data("Aperture must be above 0"));
        }
        if let Some(distance) = self.focus_distance {
            if distance < 0.0 {
                return Err(err_invalid_data("Focus distance cannot be negative."));
            }
        }
        if self.focal_length < 0.0 {
            return Err(err_invalid_data("Focal length cannot be negative."));
        }

        Ok(())
    }
}

#[derive(Deserialize)]
enum JsonMaterial {
    ReferenceToName(String),
    Literal(JsonMaterialLiteral),
}

impl JsonMaterial {
    fn validate(&self) -> io::Result<()> {
        if let Self::ReferenceToName(name) = self {
            unsafe {
                if DEFINED_MATERIALS.contains(name) {
                    return Ok(());
                } else {
                    return Err(err_invalid_data(&format!("Undefined material: {name}")));
                }
            }
        } else if let Self::Literal(literal) = self {
            literal.validate()?;
        }

        Ok(())
    }
}

#[derive(Deserialize)]
struct JsonMaterialLiteral {
    name: Option<String>,
    type_: String,
    color: JsonColor,
    refractive_index: Option<f64>,
    fuzziness: Option<f64>,
}

impl JsonMaterialLiteral {
    fn validate(&self) -> io::Result<()> {
        if let Some(n) = &self.name {
            unsafe {
                DEFINED_MATERIALS.push(n.to_string());
            }
        }

        match self.type_.as_str() {
            "diffuse" => (),
            "metal" => {
                if let Some(fuzz) = self.fuzziness {
                    if fuzz < 0.0 {
                        return Err(err_invalid_data("Metal fuzziness cannot be negative"));
                    }
                } else {
                    return Err(err_invalid_data(
                        "Metal material needs the property \"fuzziness\".",
                    ));
                }
            }
            "dialectric" => {
                if let Some(rf) = self.refractive_index {
                    if rf < 0.0 {
                        return Err(err_invalid_data("Refractive index cannot be negative."));
                    }
                } else {
                    return Err(err_invalid_data(
                        "Dialectric material needs the property \"refractive_index\"",
                    ));
                }
            }
            invalid_material => {
                return Err(err_invalid_data(&format!(
                    "Material type \"{}\" does not exist.",
                    invalid_material
                )));
            }
        }

        self.color.validate()?;

        Ok(())
    }
}

#[derive(Deserialize)]
struct JsonColor {
    rgb: (f64, f64, f64),
    normalized: bool,
}

impl JsonColor {
    fn validate(&self) -> io::Result<()> {
        let rgb = vec![self.rgb.0, self.rgb.1, self.rgb.2];
        match self.normalized {
            true => {
                if rgb.iter().any(|num| *num < 0.0 || *num > 1.0) {
                    return Err(err_invalid_data(
                        "Normalized rgb values need to be between 0 and 1",
                    ));
                }
            }
            false => {
                if rgb.iter().any(|num| *num < 0.0 || *num > 255.0) {
                    return Err(err_invalid_data(
                        "Non normalized rgb values need to be between 0 and 255",
                    ));
                }
            }
        }

        Ok(())
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
        if self.radius < 0.0 {
            eprintln!("Warning: Sphere with negative radius created. This inverts the normals.");
        }
        self.material.validate()?;
        Ok(())
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
