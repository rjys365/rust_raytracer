use rand::prelude::ThreadRng;
use crate::math_util::{Color,Ray,Vec3};
use crate::models::hittable::HitRecord;
use super::Material;

pub struct Lambertian{
    pub albedo:Color
}

impl Material for Lambertian{
    fn scatter(&self,_r_in:&Ray,rec:&HitRecord,rand:&mut ThreadRng)->Option<(Color,Ray)> {
        let scatter_direction_t=rec.normal+Vec3::random_unit_vector(rand);
        let scatter_direction=if scatter_direction_t.near_zero() {rec.normal} else {scatter_direction_t};
        Some((self.albedo,Ray::from(&rec.p,&scatter_direction)))
    }
}
