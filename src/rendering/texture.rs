use super::Color;

pub trait Texture {
    fn value(&self, u: f64, v: f64) -> Color;
}

pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    fn new(color: Color) -> SolidColor {
        SolidColor { color }
    }

    fn new_by_rgb(red: f64, green: f64, blue: f64) -> SolidColor {
        SolidColor::new(Color::new(red, green, blue))
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64) -> Color {
        self.color
    } 
}
