use super::{Color, Point3};

pub trait Texture: CloneTexture + Send + Sync {
    fn value(&self, u: f64, v: f64, hit_point: Point3) -> Color;
}

pub trait CloneTexture {
    fn clone_texture(&self) -> Box<dyn Texture>;
}

impl<T: Texture + Clone + 'static> CloneTexture for T {
    fn clone_texture(&self) -> Box<dyn Texture> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Texture> {
    fn clone(&self) -> Self {
        self.clone_texture()
    }
}

#[derive(Clone)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color }
    }

    pub fn from_rgb(red: f64, green: f64, blue: f64) -> SolidColor {
        SolidColor::new(Color::new(red, green, blue))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _hit_point: Point3) -> Color {
        self.color
    }
}

#[derive(Clone)]
pub struct CheckerBoard {
    even: Box<dyn Texture>,
    odd: Box<dyn Texture>,
}

impl CheckerBoard {
    pub fn new(even: Box<dyn Texture>, odd: Box<dyn Texture>) -> CheckerBoard {
        CheckerBoard { even, odd }
    }

    pub fn from_colors(even: Color, odd: Color) -> CheckerBoard {
        CheckerBoard { even: Box::new(SolidColor::new(even)), odd: Box::new(SolidColor::new(odd)) }
    }
}

impl Texture for CheckerBoard {
    fn value(&self, u: f64, v: f64, hit_point: Point3) -> Color {
        let sines = [hit_point.x(), hit_point.y(), hit_point.z()].map(|coord| (10.0 * coord).sin()).into_iter().reduce(|acc, e| acc * e).unwrap();
        match sines < 0.0 {
            true => self.odd.value(u, v, hit_point),
            false => self.even.value(u, v, hit_point),
        }
    }
}
