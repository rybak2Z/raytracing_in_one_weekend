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
    let from = json_camera.look_from;
    let at = json_camera.look_at;
    let up = json_camera.view_up_direction;

    let camera_config = CameraConfiguration {
        look_from: Point3::new(from.x, from.y, from.z),
        look_at: Point3::new(at.x, at.y, at.z),
        view_up: Vec3::new(up.x, up.y, up.z),
        vertical_fov: json_camera.vertical_fov_degrees,
        aperture: json_camera.aperture,
        focus_distance: json_camera.focus_distance,
        focal_length: json_camera.focal_length,
        start_time: Some(json_camera.start_time),
        end_time: Some(json_camera.end_time),
    };

    Camera::new(camera_config)
}

fn create_world(json_objects: Vec<JsonSphere>) -> HittableList {
    let mut world = HittableList::default();
    for json_sphere in json_objects {
        world.add(json_sphere.to_internal());
    }
    world
}
