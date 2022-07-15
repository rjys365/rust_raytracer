pub mod perlin;

use self::perlin::Perlin;
use super::Texture;
use crate::math_util::Color;
use crate::math_util::Point3;

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: Perlin::new(),
            scale: 1.0,
        }
    }
    pub fn new_scale(scale: f64) -> Self {
        Self {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Default for NoiseTexture {
    fn default() -> Self {
        Self::new()
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * self.noise.noise(&(p * self.scale))
    }
}
