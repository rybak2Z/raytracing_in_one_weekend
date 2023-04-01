use serde::Deserialize;

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
    fuzziness: Option<f64>
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
