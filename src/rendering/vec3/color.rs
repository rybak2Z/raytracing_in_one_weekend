mod premade;

pub use premade::*;

use super::Vec3;

use std::fmt::Display;
use std::ops;

#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    rgb: [f64; 3],
}

impl Color {
    pub const fn new(r: f64, g: f64, b: f64) -> Color {
        Color { rgb: [r, g, b] }
    }

    pub fn r(&self) -> f64 {
        self.rgb[0]
    }

    pub fn g(&self) -> f64 {
        self.rgb[1]
    }

    pub fn b(&self) -> f64 {
        self.rgb[2]
    }

    pub fn set_r(&mut self, new_red: f64) {
        self.rgb[0] = new_red;
    }

    pub fn set_g(&mut self, new_green: f64) {
        self.rgb[1] = new_green;
    }

    pub fn set_b(&mut self, new_blue: f64) {
        self.rgb[2] = new_blue;
    }

    pub fn random() -> Color {
        let random_vec = Vec3::random();
        Color::new(random_vec.x(), random_vec.y(), random_vec.z())
    }

    pub fn random_range(min: f64, max: f64) -> Color {
        let random_vec = Vec3::random_range(min, max);
        Color::new(random_vec.x(), random_vec.y(), random_vec.z())
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.r(), self.g(), self.b())
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r() + rhs.r(), self.g() + rhs.g(), self.b() + rhs.b())
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl ops::Mul for Color {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Color::new(self.r() * rhs.r(), self.g() * rhs.g(), self.b() * rhs.b())
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color::new(self.r() * rhs, self.g() * rhs, self.b() * rhs)
    }
}

impl ops::Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        Color::new(self * rhs.r(), self * rhs.g(), self * rhs.b())
    }
}

impl ops::MulAssign for Color {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl ops::Div<f64> for Color {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl ops::DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}
