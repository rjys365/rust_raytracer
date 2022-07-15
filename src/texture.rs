use crate::math_util::{Color, Point3};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}

pub mod checker_texture;
pub mod image_texture;
pub mod solid_color;
