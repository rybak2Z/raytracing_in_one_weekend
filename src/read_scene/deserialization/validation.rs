use super::{
    JsonCamera, JsonColor, JsonMaterial, JsonMaterialLiteral, JsonMaterialOptions, JsonSphere,
    Scene,
};

use crate::config::err_invalid_data;
use crate::rendering::material::*;

use once_cell::sync::Lazy;

use std::collections::HashMap;
use std::io;

pub static mut DEFINED_MATERIALS: Lazy<HashMap<String, Box<dyn Material>>> =
    Lazy::new(HashMap::default);

pub trait Validate {
    fn validate(&self) -> io::Result<()>;
}

impl Validate for Scene {
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

impl Validate for JsonCamera {
    fn validate(&self) -> io::Result<()> {
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

impl Validate for JsonMaterial {
    fn validate(&self) -> io::Result<()> {
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
}

impl Validate for JsonMaterialLiteral {
    fn validate(&self) -> io::Result<()> {
        match self.type_ {
            JsonMaterialOptions::Diffuse => self.validate_diffuse()?,
            JsonMaterialOptions::Metal => self.validate_metal()?,
            JsonMaterialOptions::Dialectric => self.validate_dialectric()?,
        }

        if let Some(n) = &self.name {
            self.add_to_defined_materials(n);
        }

        Ok(())
    }
}

impl JsonMaterialLiteral {
    fn validate_diffuse(&self) -> io::Result<()> {
        if let Some(c) = &self.color {
            c.validate()?;
        } else {
            return Err(err_invalid_data("Diffuse material needs a color."));
        }

        Ok(())
    }

    fn validate_metal(&self) -> io::Result<()> {
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

        Ok(())
    }

    fn validate_dialectric(&self) -> io::Result<()> {
        if let Some(rf) = self.refractive_index {
            if rf < 0.0 {
                return Err(err_invalid_data("Refractive index cannot be negative."));
            }
        } else {
            return Err(err_invalid_data(
                "Dialectric material needs the property \"refractive_index\"",
            ));
        }

        Ok(())
    }

    fn add_to_defined_materials(&self, name: &str) {
        unsafe {
            DEFINED_MATERIALS.insert(name.to_string(), self.to_material());
        }
    }
}

impl Validate for JsonColor {
    fn validate(&self) -> io::Result<()> {
        let rgb = vec![self.rgb.0, self.rgb.1, self.rgb.2];
        match self.normalized {
            true => self.validate_normalized(rgb)?,
            false => self.validate_non_normalized(rgb)?,
        }

        Ok(())
    }
}

impl JsonColor {
    fn validate_normalized(&self, rgb: Vec<f64>) -> io::Result<()> {
        if rgb.iter().any(|num| *num < 0.0 || *num > 1.0) {
            return Err(err_invalid_data(
                "Normalized rgb values need to be between 0 and 1",
            ));
        }

        Ok(())
    }

    fn validate_non_normalized(&self, rgb: Vec<f64>) -> io::Result<()> {
        if rgb.iter().any(|num| *num < 0.0 || *num > 255.0) {
            return Err(err_invalid_data(
                "Non normalized rgb values need to be between 0 and 255",
            ));
        }

        Ok(())
    }
}

impl Validate for JsonSphere {
    fn validate(&self) -> io::Result<()> {
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
}
