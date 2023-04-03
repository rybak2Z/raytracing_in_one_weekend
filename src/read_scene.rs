mod deserialization;

use deserialization::*;

use crate::rendering::{camera::*, HittableList, Point3, Vec3};

use std::io;

const SCENE_PATH: &str = "default_scene.json";

pub fn read_scene() -> io::Result<(HittableList, Camera)> {
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
    let start_time = json_camera.start_time;
    let end_time = json_camera.end_time;

    let camera_config = CameraConfiguration {
        look_from,
        look_at,
        view_up,
        vertical_fov: json_camera.vertical_fov_degrees,
        aperture: json_camera.aperture,
        focus_distance: json_camera.focus_distance,
        focal_length: json_camera.focal_length,
        start_time: Some(start_time),
        end_time: Some(end_time),
    };

    Camera::new(camera_config)
}

fn create_world(json_objects: Vec<JsonSphere>) -> HittableList {
    let mut world = HittableList::default();
    for json_sphere in json_objects {
        world.add(json_sphere.to_sphere());
    }
    world
}
