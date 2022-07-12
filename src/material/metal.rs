use rand::prelude::ThreadRng;
use crate::math_util::{Color,Ray,Vec3,dot};
use crate::models::hittable::HitRecord;
use super::Material;

pub struct Metal{
    pub albedo:Color
}

impl Material for Metal{
    fn scatter(&self,r_in:&Ray,rec:&HitRecord,_rand:&mut ThreadRng)->Option<(Color,Ray)> {
        let reflected=Vec3::reflect(r_in.get_direction(), &rec.normal);
        let scattered=Ray::from(&rec.p,&reflected);
        let attenuation=self.albedo;
        if dot(scattered.get_direction(),&rec.normal)>0.0 {Some((attenuation,scattered))} else {None}
    }
}
