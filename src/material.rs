use rand::prelude::ThreadRng;

use crate::math_util::{Ray,Color};
use crate::models::hittable::HitRecord;

pub trait Material{
    //(attenuation,scattered)
    fn scatter(&self,r_in:&Ray,rec:&HitRecord,rand:&mut ThreadRng)->Option<(Color,Ray)>;
}

pub mod lambertian;
pub mod metal;