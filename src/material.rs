use crate::math_util::{Color, Point3, Ray};
use crate::models::hittable::HitRecord;

pub trait Material {
    //(attenuation,scattered)
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::default()
    }
}

pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;
