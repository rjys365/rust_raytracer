use crate::math_util::Color;

use super::Texture;

pub struct SolidColor {
    pub color_value: Color,
}

impl SolidColor {
    pub fn new(color_value: Color) -> Self {
        Self { color_value }
    }
    pub fn new_rgb(r: f64, g: f64, b: f64) -> Self {
        Self {
            color_value: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: crate::math_util::Point3) -> Color {
        self.color_value
    }
}
