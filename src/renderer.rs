use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::writing::FileWriter;
use crate::{Color, Hittable, Interval, Ray};

use std::io;
use std::thread::{self, JoinHandle};

pub struct ImageBuffer {
    buffer: Vec<Color>,
}

impl ImageBuffer {
    fn new(image_width: usize, image_height: usize) -> Self {
        let capacity = image_width * image_height;
        let buffer = Vec::with_capacity(capacity);
        Self { buffer }
    }

    /// Merge multiple other ImageBuffer struct into self
    fn merge(&mut self, others: Vec<ImageBuffer>) {
        for image_buf in others.iter() {
            for (i, color) in image_buf.buffer.iter().enumerate() {
                self.buffer[i] += *color;
            }
        }
    }

    fn write_pixel(&mut self, color: Color) {
        self.buffer.push(color);
    }

    pub fn get_buffer(&self) -> &Vec<Color> {
        &self.buffer
    }
}

pub struct Renderer {
    world: HittableList,
    camera: Camera,
    samples_per_pixel: u32,
    max_ray_depth: u32,
}

impl Renderer {
    pub fn new(
        world: HittableList,
        camera: Camera,
        samples_per_pixel: u32,
        max_ray_depth: u32,
    ) -> Self {
        Self {
            world,
            camera,
            samples_per_pixel,
            max_ray_depth,
        }
    }

    pub fn start(&self, threads: u32) -> io::Result<()> {
        let thread_samples = self.calculate_sample_distribution(threads);
        let (main_thread_samples, thread_samples) = thread_samples.split_first().unwrap();

        let threads_left = threads - 1; // because the main thread will also do rendering
        let thread_handles = self.spawn_render_threads(threads_left, thread_samples);
        let main_buffer = Renderer::render(
            &self.world,
            &self.camera,
            *main_thread_samples,
            self.max_ray_depth,
        );
        let main_buffer = self.merge_thread_buffers(main_buffer, thread_handles);

        let mut file_writer =
            FileWriter::new(self.camera.image_width(), self.camera.image_height())?;
        file_writer.write_image(main_buffer, self.samples_per_pixel)?;

        Ok(())
    }

    fn calculate_sample_distribution(&self, threads: u32) -> Vec<u32> {
        let samples_per_pixel_per_thread = self.samples_per_pixel / threads;
        let samples_left = self.samples_per_pixel % threads;

        let mut thread_samples = vec![samples_per_pixel_per_thread; threads as usize];

        #[allow(clippy::needless_range_loop)]
        for i in 0..(samples_left as usize) {
            thread_samples[i] += 1;
        }

        thread_samples
    }

    fn spawn_render_threads(
        &self,
        threads: u32,
        thread_samples: &[u32],
    ) -> Vec<JoinHandle<ImageBuffer>> {
        let mut thread_handles = vec![];

        #[allow(clippy::needless_range_loop)]
        for i in 0..(threads as usize) {
            let world_clone = self.world.clone();
            let camera_clone = self.camera.clone();
            let max_ray_depth = self.max_ray_depth;

            let samples_per_pixel = thread_samples[i];

            let handle = thread::spawn(move || {
                Renderer::render(
                    &world_clone,
                    &camera_clone,
                    samples_per_pixel,
                    max_ray_depth,
                )
            });

            thread_handles.push(handle);
        }

        thread_handles
    }

    fn merge_thread_buffers(
        &self,
        mut main_buffer: ImageBuffer,
        thread_handles: Vec<JoinHandle<ImageBuffer>>,
    ) -> ImageBuffer {
        let mut buffers = vec![];
        for handle in thread_handles.into_iter() {
            let returned_buffer = handle.join().unwrap();
            buffers.push(returned_buffer);
        }

        main_buffer.merge(buffers);

        main_buffer
    }

    fn render(
        world: &HittableList,
        camera: &Camera,
        samples_per_pixel: u32,
        max_ray_depth: u32,
    ) -> ImageBuffer {
        let mut buffer = ImageBuffer::new(
            camera.image_width() as usize,
            camera.image_height() as usize,
        );

        for row in 0..camera.image_height() {
            for col in 0..camera.image_width() {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _sample in 1..=samples_per_pixel {
                    let ray = camera.get_ray(row, col);
                    pixel_color += Renderer::ray_color(&ray, max_ray_depth, world);
                }

                buffer.write_pixel(pixel_color);
            }
        }

        buffer
    }

    fn ray_color(ray: &Ray, depth: u32, world: &impl Hittable) -> Color {
        if depth == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_rec) = world.hit(ray, Interval::new(0.001, f32::INFINITY)) {
            // At this point, the hit record should have a material, so we can unwrap
            let material = hit_rec.material.as_ref().unwrap();

            let color = if let Some(scatter) = material.scatter(ray, &hit_rec) {
                scatter.attenuation * Self::ray_color(&scatter.ray, depth - 1, world)
            } else {
                Color::new(0.0, 0.0, 0.0)
            };

            return color;
        }

        // Gradient background
        let white = Color::new(1.0, 1.0, 1.0);
        let blue = Color::new(0.5, 0.7, 1.0);
        let direction = ray.direction().normalized();
        let lerp_factor = 0.5 * (direction.y + 1.0);
        (1.0 - lerp_factor) * white + lerp_factor * blue
    }
}
