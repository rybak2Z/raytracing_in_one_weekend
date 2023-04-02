use super::{
    JsonCamera, JsonColor, JsonMaterial, JsonMaterialLiteral, JsonMaterialOptions, JsonSphere,
    Scene,
};

use crate::config::err_invalid_data;
use crate::rendering::{
    material::*, moving_sphere::MovingSphere, sphere::Sphere, Color, Hittable, Point3,
};

use once_cell::sync::Lazy;

use std::collections::HashMap;
use std::io;

pub static mut DEFINED_MATERIALS: Lazy<HashMap<String, Box<dyn Material>>> =
    Lazy::new(HashMap::default);

impl Scene {
    pub fn validate(&self) -> io::Result<()> {
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

impl JsonCamera {
    pub fn validate(&self) -> io::Result<()> {
        if self.vertical_fov_degrees <= 0.0 || self.vertical_fov_degrees >= 180.0 {
            return Err(err_invalid_data("FOV must be between 0 and 180 degrees"));
        }
        if self.aperture < 0.0 {
            return Err(err_invalid_data("Aperture cannot be negative."));
        }
        if let Some(distance) = self.focus_distance {
            if distance < 0.0 {
                return Err(err_invalid_data("Focus distance cannot be negative."));
            }
        }
        if self.focal_length < 0.0 {
            return Err(err_invalid_data("Focal length cannot be negative."));
        }
        if self.start_time > self.end_time {
            return Err(err_invalid_data("End time cannot be before start time"));
        }

        Ok(())
    }
}

impl JsonMaterial {
    pub fn validate(&self) -> io::Result<()> {
        if let Self::ReferenceToName(name) = self {
            unsafe {
                if DEFINED_MATERIALS.contains_key(name) {
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

    pub fn to_material(&self) -> Box<dyn Material> {
        match self {
            Self::ReferenceToName(name) => unsafe { DEFINED_MATERIALS.get(name).unwrap().clone() },
            Self::Literal(literal) => literal.to_material(),
        }
    }
}

impl JsonMaterialLiteral {
    pub fn validate(&self) -> io::Result<()> {
        match self.type_ {
            JsonMaterialOptions::Diffuse => {
                if let Some(c) = &self.color {
                    c.validate()?;
                } else {
                    return Err(err_invalid_data("Diffuse material needs a color."));
                }
            }
            JsonMaterialOptions::Metal => {
                if let Some(fuzz) = self.fuzziness {
                    if fuzz < 0.0 {
                        return Err(err_invalid_data("Metal fuzziness cannot be negative"));
                    }
                } else {
                    return Err(err_invalid_data(
                        "Metal material needs the property \"fuzziness\".",
                    ));
                }
                if let Some(c) = &self.color {
                    c.validate()?;
                } else {
                    return Err(err_invalid_data("Metal material needs a color."));
                }
            }
            JsonMaterialOptions::Dialectric => {
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
        }

        if let Some(n) = &self.name {
            self.add_to_defined_materials(n);
        }

        Ok(())
    }

    fn add_to_defined_materials(&self, name: &str) {
        unsafe {
            DEFINED_MATERIALS.insert(name.to_string(), self.to_material());
        }
    }

    pub fn to_material(&self) -> Box<dyn Material> {
        match self.type_ {
            JsonMaterialOptions::Diffuse => {
                Box::new(Lambertian::new(self.color.as_ref().unwrap().to_color()))
            }
            JsonMaterialOptions::Metal => Box::new(Metal::new(
                self.color.as_ref().unwrap().to_color(),
                self.fuzziness.unwrap(),
            )),
            JsonMaterialOptions::Dialectric => {
                Box::new(Dialectric::new(self.refractive_index.unwrap()))
            }
        }
    }
}

impl JsonColor {
    pub fn validate(&self) -> io::Result<()> {
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

    pub fn to_color(&self) -> Color {
        let (mut r, mut g, mut b) = self.rgb;
        if !self.normalized {
            r /= 255.0;
            g /= 255.0;
            b /= 255.0;
        }
        Color::new(r, g, b)
    }
}

impl JsonSphere {
    pub fn validate(&self) -> io::Result<()> {
        if self.radius < 0.0 {
            eprintln!("Warning: Sphere with negative radius created. This inverts the normals.");
        }
        if let Some(mov) = &self.movement {
            if mov.start_time > mov.end_time {
                return Err(err_invalid_data("End time cannot be before start time."));
            }
        }
        self.material.validate()?;
        Ok(())
    }

    pub fn to_sphere(&self) -> Box<dyn Hittable> {
        let (x, y, z) = (self.coordinates.0, self.coordinates.1, self.coordinates.2);
        let point = Point3::new(x, y, z);
        if let Some(mov) = &self.movement {
            let target_point = Point3::new(mov.target.0, mov.target.1, mov.target.2);
            Box::new(MovingSphere::new(
                point,
                target_point,
                mov.start_time,
                mov.end_time,
                self.radius,
                self.material.to_material(),
            ))
        } else {
            Box::new(Sphere::new(point, self.radius, self.material.to_material()))
        }
    }
}
