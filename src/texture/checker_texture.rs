use std::rc::Rc;

use crate::math_util::Color;

use super::{solid_color::SolidColor, Texture};

pub struct CheckerTexture {
    pub odd: Rc<dyn Texture>,
    pub even: Rc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(even: Rc<dyn Texture>, odd: Rc<dyn Texture>) -> Self {
        Self { odd, even }
    }
    pub fn new_color(color_even: Color, color_odd: Color) -> Self {
        Self {
            even: Rc::new(SolidColor::new(color_even)),
            odd: Rc::new(SolidColor::new(color_odd)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: crate::math_util::Point3) -> Color {
        let sins = f64::sin(10.0 * p.x()) * f64::sin(10.0 * p.y()) * f64::sin(10.0 * p.z());
        if sins < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
