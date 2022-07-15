use std::rc::Rc;

use crate::{
    math_util::{Color, Point3, Ray},
    models::hittable::HitRecord,
    texture::{solid_color::SolidColor, Texture},
};

use super::Material;

pub struct DiffuseLight {
    pub emit: Rc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Rc<dyn Texture>) -> Self {
        Self { emit }
    }
    pub fn new_solid_color(c: Color) -> Self {
        Self {
            emit: Rc::new(SolidColor::new(c)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Color, Ray)> {
        None
    }
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, *p)
    }
}
