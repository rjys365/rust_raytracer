use super::Material;
use crate::math_util::{dot, Color, Ray, Vec3};
use crate::models::hittable::HitRecord;

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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = Vec3::reflect(r_in.direction(), &rec.normal);
        let scattered = Ray::new_time(
            rec.p,
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            r_in.time(),
        );
        let attenuation = self.albedo;
        if dot(scattered.direction(), &rec.normal) > 0.0 {
            Some((attenuation, scattered))
        } else {
            None
        }
    }
}
