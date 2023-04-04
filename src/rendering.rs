pub mod camera;
pub mod hit_detection;
pub mod material;
pub mod moving_sphere;
pub mod sphere;
pub mod vec3;

mod aabb;
mod bvh_node;
mod coordinate_iterator;
mod ray;

pub use aabb::AABB;
pub use bvh_node::BvhNode;
pub use hit_detection::{HitRecord, Hittable, HittableList};
pub use ray::Ray;
pub use vec3::{
    color::{self, Color},
    Point3, Vec3,
};

use camera::Camera;
use coordinate_iterator::CoordinateIterator;
use material::Material;

use crate::config::*;
use crate::writing::WritingSynchronizer;

use rand::prelude::*;

use std::io;
use std::sync::{
    mpsc::{self, Receiver, Sender},
    Arc, Mutex,
};
use std::thread::{self, JoinHandle};

struct RenderingTools<'a> {
    bvh: &'a BvhNode,
    camera: &'a Camera,
    rng: ThreadRng,
}

impl RenderingTools<'_> {
    pub fn new<'a>(bvh: &'a BvhNode, camera: &'a Camera) -> RenderingTools<'a> {
        RenderingTools {
            bvh,
            camera,
            rng: thread_rng(),
        }
    }
}

pub fn render(bvh: BvhNode, camera: Camera) -> io::Result<()> {
    let coordinate_iterator = Arc::new(Mutex::new(CoordinateIterator::new()));
    let (tx, rx) = mpsc::channel::<(Color, (u32, u32))>();

    let mut handles: Vec<JoinHandle<()>> = vec![];
    for _ in 0..(THREADS.get().unwrap() - 1) {
        let bvh_copy = bvh.clone();
        let camera_copy = camera.clone();
        let tx_copy = tx.clone();
        let shared_iterator = Arc::clone(&coordinate_iterator);

        let handle = thread::spawn(move || {
            do_work(bvh_copy, camera_copy, shared_iterator, tx_copy);
        });

        handles.push(handle);
    }

    let mut writing_sync = WritingSynchronizer::new();
    main_thread_work(bvh, camera, &mut writing_sync, coordinate_iterator, &rx)?;

    finish(handles, writing_sync, rx, tx)?;

    Ok(())
}

fn do_work(
    bvh: BvhNode,
    camera: Camera,
    shared_iterator: Arc<Mutex<CoordinateIterator>>,
    tx: Sender<(Color, (u32, u32))>,
) {
    let mut render_tools = RenderingTools::new(&bvh, &camera);

    while let Some((row, col)) = get_next_coordinates(&shared_iterator) {
        let pixel_color = calculate_pixel_color(row, col, &mut render_tools);
        tx.send((pixel_color, (row, col))).unwrap();
    }
}

fn main_thread_work(
    bvh: BvhNode,
    camera: Camera,
    writing_sync: &mut WritingSynchronizer,
    shared_iterator: Arc<Mutex<CoordinateIterator>>,
    rx: &Receiver<(Color, (u32, u32))>,
) -> io::Result<()> {
    let mut render_tools = RenderingTools::new(&bvh, &camera);

    while !writing_sync.all_data_written() {
        while let Ok((color, (row, col))) = rx.try_recv() {
            writing_sync.write(color, row, col)?;
        }

        if !USE_MAIN_THREAD_FOR_RENDERING.get().unwrap() && *THREADS.get().unwrap() > 1 {
            continue;
        }

        let (row, col) = match get_next_coordinates(&shared_iterator) {
            Some(c) => (c.0, c.1),
            None => break,
        };
        let pixel_color = calculate_pixel_color(row, col, &mut render_tools);

        writing_sync.write(pixel_color, row, col)?;
    }

    Ok(())
}

fn get_next_coordinates(shared_iterator: &Arc<Mutex<CoordinateIterator>>) -> Option<(u32, u32)> {
    let mut coordinate_iterator = shared_iterator.lock().unwrap();
    let coords = coordinate_iterator.next()?;
    let (row, col) = coords;
    Some((row, col))
}

fn finish(
    handles: Vec<JoinHandle<()>>,
    mut writing_sync: WritingSynchronizer,
    rx: Receiver<(Color, (u32, u32))>,
    tx: Sender<(Color, (u32, u32))>,
) -> io::Result<()> {
    for handle in handles {
        handle.join().unwrap();
    }

    for (color, (row, col)) in rx.try_iter() {
        writing_sync.write(color, row, col)?;
    }
    drop(tx); // to keep channel open until now

    writing_sync.finish_writing()?;

    Ok(())
}

fn calculate_pixel_color(row: u32, col: u32, render_tools: &mut RenderingTools) -> Color {
    let accumulated_color = accumulate_pixel_color_samples(row, col, render_tools);
    let mut pixel_color = accumulated_color / *SAMPLES_PER_PIXEL.get().unwrap() as f64;
    correct_gamma(&mut pixel_color);
    pixel_color
}

fn accumulate_pixel_color_samples(row: u32, col: u32, render_tools: &mut RenderingTools) -> Color {
    let mut accumulated_color = Color::default();
    for _sample in 0..*SAMPLES_PER_PIXEL.get().unwrap() {
        accumulated_color += calculate_sample(row, col, render_tools);
    }

    accumulated_color
}

fn calculate_sample(row: u32, col: u32, render_tools: &mut RenderingTools) -> Color {
    let (u, v) = get_uv(row, col, &mut render_tools.rng);
    let ray = render_tools.camera.get_ray(u, v);
    get_ray_color(
        &ray,
        render_tools.bvh,
        *MAX_CHILD_RAYS.get().unwrap() as i32,
    )
}

fn get_uv(row: u32, col: u32, rng: &mut ThreadRng) -> (f64, f64) {
    let u = (col as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH.get().unwrap() - 1) as f64;
    let v = (row as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT.get().unwrap() - 1) as f64;
    (u, v)
}

fn get_ray_color(ray: &Ray, world: &BvhNode, depth: i32) -> Color {
    if depth == 0 {
        return Color::default();
    }

    if let Some(hit_record) = world.hit(ray, 0.0001, f64::INFINITY) {
        if let Some(scatter) = hit_record.material.scatter(ray, &hit_record) {
            return scatter.attenuation * get_ray_color(&scatter.ray, world, depth - 1);
        }
        return Color::default();
    }

    get_sky_color(ray)
}

fn get_sky_color(ray: &Ray) -> Color {
    let direction = ray.direction().normalized();
    let blend_factor = 0.5 * (direction.y() + 1.0);

    let white_part = (1.0 - blend_factor) * color::WHITE;
    let blue_part = blend_factor * color::SKY;

    white_part + blue_part
}

fn correct_gamma(color: &mut Color) {
    color.set_r(color.r().sqrt());
    color.set_g(color.g().sqrt());
    color.set_b(color.b().sqrt());
}
