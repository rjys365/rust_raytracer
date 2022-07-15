use std::rc::Rc;

use super::Material;
use crate::math_util::{Color, Ray, Vec3};
use crate::models::hittable::HitRecord;
use crate::texture::solid_color::SolidColor;
use crate::texture::Texture;

pub struct Lambertian {
    pub albedo: Rc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let scatter_direction_t = rec.normal + Vec3::random_unit_vector();
        let scatter_direction = if scatter_direction_t.near_zero() {
            rec.normal
        } else {
            scatter_direction_t
        };
        Some((
            self.albedo.value(rec.u, rec.v, rec.p),
            Ray::new_time(rec.p, scatter_direction, r_in.time()),
        ))
    }
}

impl Lambertian {
    pub fn new_solid_color(albedo: Color) -> Self {
        Self {
            albedo: Rc::new(SolidColor::new(albedo)),
        }
    }
    pub fn new_texture(texture: Rc<dyn Texture>) -> Self {
        Self { albedo: texture }
    }
}
