use super::Color;

pub trait Texture: CloneTexture + Send + Sync {
    fn value(&self, u: f64, v: f64) -> Color;
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
    fn value(&self, _u: f64, _v: f64) -> Color {
        self.color
    }
}
