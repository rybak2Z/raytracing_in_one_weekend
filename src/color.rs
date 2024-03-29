#![allow(dead_code)]

use crate::config::MAX_VALUE;

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

    pub fn pixel_format(self) -> String {
        let factor = MAX_VALUE as f32 + 0.999;
        format!(
            "{} {} {}\n",
            (self.r * factor) as u32,
            (self.g * factor) as u32,
            (self.b * factor) as u32,
        )
    }
}
