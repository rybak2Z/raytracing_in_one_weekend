use super::validation::DEFINED_MATERIALS;
use super::*;

use crate::rendering::{
    material::*, moving_sphere::MovingSphere, sphere::Sphere, Color, Hittable, Point3,
};

impl JsonMaterial {
    pub fn to_material(&self) -> Box<dyn Material> {
        match self {
            Self::ReferenceToName(name) => unsafe { DEFINED_MATERIALS.get(name).unwrap().clone() },
            Self::Literal(literal) => literal.to_material(),
        }
    }
}

impl JsonMaterialLiteral {
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
