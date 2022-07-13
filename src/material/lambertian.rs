use super::Material;
use crate::math_util::{Color, Ray, Vec3};
use crate::models::hittable::HitRecord;

pub struct Lambertian {
    pub albedo: Color,
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
            self.albedo,
            Ray::new_time(rec.p, scatter_direction, r_in.time()),
        ))
    }
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}
