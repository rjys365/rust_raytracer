use super::Material;
use crate::math_util::{dot, rand_double, Color, Ray, Vec3};
use crate::models::hittable::HitRecord;

pub struct Dieletric {
    pub ir: f64, //index of refraction
}

impl Dieletric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powi(1.0 - cosine, 5)
    }
}

impl Material for Dieletric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();
        //let refracted = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        let cos_theta = f64::min(dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_reflect = (refraction_ratio * sin_theta) > 1.0;
        let direction =
            if cannot_reflect || Self::reflectance(cos_theta, refraction_ratio) > rand_double() {
                Vec3::reflect(&unit_direction, &rec.normal)
            } else {
                Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
            };
        Some((attenuation, Ray::new_time(rec.p, direction, r_in.time())))
    }
}
