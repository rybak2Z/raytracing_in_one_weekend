use super::{Basis, Camera, Defocus, Image, Orientation, RenderOptions, Viewport};

use crate::{Point3, Vec3};

pub struct CameraBuilder {
    pub image_width: u32,
    pub aspect_ratio: f32,
    pub vertical_fov: f32,
    pub position: Point3,
    pub look_at: Point3,
    pub view_up: Vec3,
    pub focus_distance: f32,
    pub defocus_angle: f32,
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self {
            image_width: 100,
            aspect_ratio: 1.0,
            vertical_fov: 90.0,
            position: Point3::zero(),
            look_at: Point3::new(0.0, 0.0, 1.0),
            view_up: Vec3::new(0.0, 1.0, 0.0),
            focus_distance: 1.0,
            defocus_angle: 0.0,
            samples_per_pixel: 10,
            max_depth: 3,
        }
    }
}

impl CameraBuilder {
    pub fn finalize(self) -> Camera {
        let image = self.calculate_image();
        let basis = self.calculate_basis();
        let viewport = self.calculate_viewport(&image, &basis);
        let defocus = self.calculate_defocus(&basis);

        let orientation = Orientation {
            look_at: self.look_at,
            view_up: self.view_up,
        };

        let render_options = RenderOptions {
            samples_per_pixel: self.samples_per_pixel,
            max_depth: self.max_depth,
        };

        Camera {
            position: self.position,
            vertical_fov: self.vertical_fov,
            orientation,
            basis,
            defocus,
            image,
            viewport,
            render_options,
        }
    }

    fn calculate_image(&self) -> Image {
        let image_height = (self.image_width as f32 / self.aspect_ratio).round() as u32;
        let image_height = image_height.max(1);

        Image {
            width: self.image_width,
            height: image_height,
            aspect_ratio: self.aspect_ratio,
        }
    }

    fn calculate_basis(&self) -> Basis {
        let back = (self.position - self.look_at).normalized();
        let right = Vec3::cross(self.view_up, back).normalized();
        let up = Vec3::cross(back, right);

        Basis { up, right, back }
    }

    fn calculate_viewport(&self, image: &Image, basis: &Basis) -> Viewport {
        // Determine viewport dimensions
        let theta = self.vertical_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * self.focus_distance;
        let actual_aspect_ratio = image.width as f32 / image.height as f32;
        let viewport_width = viewport_height * actual_aspect_ratio;

        // Vectors across the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * basis.right;
        let viewport_v = viewport_height * (-basis.up);

        // Horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image.width as f32;
        let pixel_delta_v = viewport_v / image.height as f32;

        // Calculate position of top left pixel
        let viewport_center = self.position - self.focus_distance * basis.back;
        let viewport_top_left = viewport_center - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_top_left = viewport_top_left + pixel_delta_u / 2.0 + pixel_delta_v / 2.0;

        Viewport {
            width: viewport_width,
            height: viewport_height,
            pixel_top_left,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    fn calculate_defocus(&self, basis: &Basis) -> Defocus {
        let defocus_radius = self.focus_distance * (self.defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = basis.right * defocus_radius;
        let defocus_disk_v = basis.up * defocus_radius;

        Defocus {
            focus_distance: self.focus_distance,
            angle: self.defocus_angle,
            disk_u: defocus_disk_u,
            disk_v: defocus_disk_v,
        }
    }
}
