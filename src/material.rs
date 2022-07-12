use rand::prelude::ThreadRng;

use crate::math_util::{Color, Ray};
use crate::models::hittable::HitRecord;

pub trait Material {
    //(attenuation,scattered)
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rand: &mut ThreadRng) -> Option<(Color, Ray)>;
}

pub mod dielectric;
pub mod lambertian;
pub mod metal;
