use super::Material;
use crate::math_util::{dot, Color, Ray, Vec3};
use crate::models::hittable::HitRecord;
use rand::prelude::ThreadRng;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, rand: &mut ThreadRng) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(r_in.get_direction(), &rec.normal);
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(rand),
        );
        let attenuation = self.albedo;
        if dot(scattered.get_direction(), &rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
