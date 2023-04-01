use super::{Scene, JsonCamera, JsonMaterial, JsonMaterialLiteral, JsonColor, JsonSphere};

use crate::config::err_invalid_data;

use std::io;

static mut DEFINED_MATERIALS: Vec<String> = Vec::new();

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

impl JsonMaterial {
    pub fn validate(&self) -> io::Result<()> {
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

impl JsonMaterialLiteral {
    pub fn validate(&self) -> io::Result<()> {
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
}

impl JsonSphere {
    pub fn validate(&self) -> io::Result<()> {
        if self.radius < 0.0 {
            eprintln!("Warning: Sphere with negative radius created. This inverts the normals.");
        }
        self.material.validate()?;
        Ok(())
    }
}
