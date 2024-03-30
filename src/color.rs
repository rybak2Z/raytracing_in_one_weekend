#![allow(dead_code)]

use crate::MAX_VALUE;

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Color {
            r: red,
            g: green,
            b: blue,
        }
    }

    pub fn pixel_format(self, samples: u32) -> String {
        let Color {
            mut r,
            mut g,
            mut b,
        } = self;
        let downscale = 1.0 / samples as f32;
        r *= downscale;
        g *= downscale;
        b *= downscale;

        // The +1 makes it so the interval of numbers that would be scaled to
        // MAX_VALUE is the same size as the intervals for all other values.
        // Without this, a color could only be scaled up to MAX_VALUE when it
        // is exactly 1.0. To avoid writing values above MAX_VALUE, the
        // scaled up colors get clamped below.
        let upscale = (MAX_VALUE + 1) as f32;
        format!(
            "{} {} {}\n",
            ((r * upscale) as u32).clamp(0, MAX_VALUE),
            ((g * upscale) as u32).clamp(0, MAX_VALUE),
            ((b * upscale) as u32).clamp(0, MAX_VALUE),
        )
    }
}

impl Add for Color {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Color::new(self.r + rhs.r, self.g + rhs.g, self.b + rhs.b)
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Mul<f32> for Color {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Color::new(self.r * rhs, self.g * rhs, self.b * rhs)
    }
}

impl Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl Div<f32> for Color {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        *self *= 1.0 / rhs;
    }
}
