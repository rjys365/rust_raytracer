use crate::math_util::{Color, Ray, Point3};
use crate::models::hittable::HitRecord;

pub trait Material {
    //(attenuation,scattered)
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)>;
    fn emitted(&self,u:f64,v:f64,p:&Point3)->Color{
        Color::default()
    }
}

pub mod dielectric;
pub mod lambertian;
pub mod metal;
pub mod diffuse_light;
