use super::validation::DEFINED_MATERIALS;
use super::*;

use crate::rendering::{
    material::*, moving_sphere::MovingSphere, sphere::Sphere, Color, HittableEnum, Point3,
};

use std::sync::Arc;

pub trait ToInternal {
    type Output;

    fn to_internal(&self) -> Self::Output;
}

impl ToInternal for JsonMaterial {
    type Output = Arc<MaterialEnum>;

    fn to_internal(&self) -> Self::Output {
        match self {
            Self::ReferenceToName(name) => unsafe { DEFINED_MATERIALS.get(name).unwrap().clone() },
            Self::Literal(literal) => literal.to_internal(),
        }
    }
}

impl ToInternal for JsonMaterialLiteral {
    type Output = Arc<MaterialEnum>;

    fn to_internal(&self) -> Self::Output {
        match self.type_ {
            JsonMaterialOptions::Diffuse => {
                Arc::new(Lambertian::new(self.color.as_ref().unwrap().to_internal()).into())
            }
            JsonMaterialOptions::Metal => Arc::new(
                Metal::new(
                    self.color.as_ref().unwrap().to_internal(),
                    self.fuzziness.unwrap(),
                )
                .into(),
            ),
            JsonMaterialOptions::Dialectric => {
                Arc::new(Dialectric::new(self.refractive_index.unwrap()).into())
            }
        }
    }
}

impl ToInternal for JsonColor {
    type Output = Color;

    fn to_internal(&self) -> Self::Output {
        let (mut r, mut g, mut b) = self.rgb;
        if !self.normalized {
            r /= 255.0;
            g /= 255.0;
            b /= 255.0;
        }
        Color::new(r, g, b)
    }
}

impl ToInternal for JsonSphere {
    type Output = Arc<HittableEnum>;

    fn to_internal(&self) -> Self::Output {
        let (x, y, z) = (self.coordinates.0, self.coordinates.1, self.coordinates.2);
        let point = Point3::new(x, y, z);
        if let Some(mov) = &self.movement {
            let target_point = Point3::new(mov.target.0, mov.target.1, mov.target.2);
            Arc::new(
                MovingSphere::new(
                    point,
                    target_point,
                    mov.start_time,
                    mov.end_time,
                    self.radius,
                    self.material.to_internal(),
                )
                .into(),
            )
        } else {
            Arc::new(Sphere::new(point, self.radius, self.material.to_internal()).into())
        }
    }
}
